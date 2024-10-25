use crate::{
    error::LotteryError,
    state::{LotoInstruction, PoolStorageAccount, PoolStorageSeed, TicketAccountData},
};

use borsh::{to_vec, BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use solana_program::program_pack::Pack;

const DEVELOPMENT_CUT: f64 = 0.2;
const TICKET_PRICE: u64 = 50_000_000; // 0.05 SOL

pub const STAKE_POOL_MINIMUM_AMOUNT: u32 = 100_000_000;

pub fn processor(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instr = LotoInstruction::try_from_slice(instruction_data)?;
    match instr {
        LotoInstruction::InitializePool(amount) => {
            process_pool_initialization(program_id, accounts, amount)
        }
        LotoInstruction::Deposit(amount) => process_deposit(program_id, accounts, amount),
        LotoInstruction::PurchaseTicket(account_data) => {
            process_ticket_purchase(program_id, accounts, account_data)
        }
        LotoInstruction::ClosePlayerAccount => Err(LotteryError::NotImplemented.into()),
        LotoInstruction::SelectWinnersAndAirdrop() => Err(LotteryError::NotImplemented.into()),
    }
}

fn process_deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let mut accounts = accounts.into_iter();
    let payer = next_account_info(&mut accounts)?;
    let pool_vault = next_account_info(&mut accounts)?;
    let _system_program_account = next_account_info(&mut accounts)?;

    if pool_vault.owner != program_id {
        return Err(solana_program::program_error::ProgramError::IllegalOwner);
    }

    let instr = system_instruction::transfer(payer.key, pool_vault.key, amount);

    invoke(&instr, &[payer.clone(), pool_vault.clone()])?;

    Ok(())
}

fn initialize_pool_mint<'a>(
    program_id: &Pubkey,
    pool_authority_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    mint_authority_account: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    spl_token_2022_account: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    let (pool_mint_address, bump) = Pubkey::find_program_address(
        &[
            PoolStorageSeed::StakePool.as_bytes(),
            pool_authority_account.key.as_ref(),
        ],
        program_id,
    );

    if &pool_mint_address != mint_account.key {
        return Err(LotteryError::InvalidStakePoolVault.into());
    }
    let rent = Rent::get()?;

    let exempt_balance = rent.minimum_balance(spl_token_2022::state::Mint::LEN);

    if pool_authority_account.lamports() < (exempt_balance + amount) {
        return Err(LotteryError::InsufficientFunds.into());
    }

    let mint_account_instr: solana_program::instruction::Instruction =
        system_instruction::create_account(
            &pool_authority_account.key,
            &mint_account.key,
            exempt_balance + amount,
            spl_token_2022::state::Mint::LEN as u64,
            &spl_token_2022::ID,
        );

    invoke_signed(
        &mint_account_instr,
        &[
            pool_authority_account.clone(),
            mint_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    let token_init_instruction = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::ID,
        &mint_account.key,
        &mint_authority_account.key,
        None,
        0,
    )?;

    invoke_signed(
        &token_init_instruction,
        &[
            pool_authority_account.clone(),
            mint_account.clone(),
            spl_token_2022_account.clone(),
            rent_account.clone(),
        ],
        &[&[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    Ok(())
}

fn process_pool_initialization(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();

    // The stake pool authority, is the authority which can verify tickets if they are valid and proceeding to airdrop prises.
    let pool_authority_account = next_account_info(&mut accounts)?;

    // The PDA vault of the stake pool which will be used to store the tickets.
    let mint_account = next_account_info(&mut accounts)?;

    // The rent account
    let rent_account = next_account_info(&mut accounts)?;

    // The spl_token_2022 account
    let spl_token_2022_account = next_account_info(&mut accounts)?;

    // The system account
    let system_program_account = next_account_info(&mut accounts)?;

    initialize_pool_mint(
        program_id,
        pool_authority_account,
        mint_account,
        mint_account,
        rent_account,
        system_program_account,
        spl_token_2022_account,
        amount,
    )?;
    Ok(())
}

fn update_player_account<'a>(
    program_id: &Pubkey,
    player_account: &AccountInfo<'a>,
    player_pda_account: &AccountInfo<'a>,
    new_merkle_root: [u8; 32],
) -> ProgramResult {
    let mut player_account_data = player_pda_account.try_borrow_mut_data()?;

    let mut ticket_data = TicketAccountData::try_from_slice(&player_account_data)?;

    ticket_data.merkle_root = new_merkle_root;

    ticket_data.serialize(&mut &mut player_account_data[..])?;

    Ok(())
}

fn find_player_pda_account<'a>(
    program_id: &Pubkey,
    player_account: &AccountInfo<'a>,
) -> (Pubkey, u8) {
    let player_account_seeds = &[
        PoolStorageSeed::PlayerAccount.as_bytes(),
        player_account.key.as_ref(),
    ];

    Pubkey::find_program_address(player_account_seeds, program_id)
}

// This function is be responsible for creating a system account
// for the player and initializing it with the ticket data.
fn initialize_player_account<'a>(
    program_id: &Pubkey,
    player_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    player_pda_account: &AccountInfo<'a>,
    initial_merkle_root: Option<[u8; 32]>,
) -> ProgramResult {
    let rent = Rent::get()?;

    let ticket_account_data_space = std::mem::size_of::<TicketAccountData>();

    if !rent.is_exempt(player_account.lamports(), ticket_account_data_space) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    // Check if the mint account is initialized
    if mint_account.data_is_empty() {
        return Err(LotteryError::StakePoolNotInitialized.into());
    }

    // The PDA account for the player
    let (player_pda_account_address, bump_seed) =
        find_player_pda_account(program_id, player_account);

    if &player_pda_account_address != player_pda_account.key {
        return Err(LotteryError::InvalidPlayerPdaAccount.into());
    }

    // The minimum balance required to create the account
    let minimum_balance = rent.minimum_balance(ticket_account_data_space);

    let instruction = system_instruction::create_account(
        &player_account.key,
        &player_pda_account_address,
        minimum_balance,
        ticket_account_data_space as u64,
        program_id,
    );

    invoke_signed(
        &instruction,
        &[player_account.clone(), player_pda_account.clone()],
        &[&[player_account.key.as_ref(), &[bump_seed]]],
    )?;

    let mut player_account_data = player_pda_account.try_borrow_mut_data()?;

    let ticket_data = TicketAccountData {
        merkle_root: initial_merkle_root.unwrap_or([0; 32]),
        address: *player_account.key,
    };

    ticket_data.serialize(&mut &mut player_account_data[..])?;

    Ok(())
}

// This function is be repsonsible for creating a token 2022 account for the player.
fn initialize_player_token_account<'a>(
    program_id: &Pubkey,
    mint_account: &AccountInfo<'a>,
    player_account: &AccountInfo<'a>,
    player_pda_account: &AccountInfo<'a>,
) -> ProgramResult {
    let (.., bump_seed) = find_player_pda_account(program_id, player_account);
    let init_account_instr = spl_token_2022::instruction::initialize_account(
        &spl_token_2022::ID,
        &player_pda_account.key,
        &mint_account.key,
        &program_id,
    )?;

    invoke_signed(
        &init_account_instr,
        &[player_pda_account.clone(), mint_account.clone()],
        &[&[player_account.key.as_ref(), &[bump_seed]]],
    )?;

    Ok(())
}

/// Process the player initialization
/// This function will create a new account for the player and transfer the ticket price to the stake pool vault.
/// The player account will be initialized with the ticket data.
/// The player account will be owned by the program.
fn process_ticket_purchase(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_data: TicketAccountData,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();
    // Account payer
    let player_account = next_account_info(&mut accounts)?;
    // Account PDA for payer
    let player_pda_account = next_account_info(&mut accounts)?;
    // Stake pool vault
    let pool_mint_account = next_account_info(&mut accounts)?;
    // Spl 2022 token account
    let stake_pool_vault = next_account_info(&mut accounts)?;

    if !player_account.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    if player_pda_account.data_is_empty() {
        initialize_player_account(
            program_id,
            player_account,
            pool_mint_account,
            player_pda_account,
            Some(account_data.merkle_root),
        )?;
    } else {
        update_player_account(
            program_id,
            player_account,
            player_pda_account,
            account_data.merkle_root,
        )?;
    }

    initialize_player_token_account(
        program_id,
        pool_mint_account,
        player_pda_account,
        player_account,
    )?;

    let ticket_purchase_instr =
        system_instruction::transfer(player_account.key, stake_pool_vault.key, TICKET_PRICE);

    let (.., bump_seed) = find_player_pda_account(program_id, player_account);

    invoke_signed(
        &ticket_purchase_instr,
        &[player_account.clone(), stake_pool_vault.clone()],
        &[&[player_account.key.as_ref(), &[bump_seed]]],
    )?;

    Ok(())
}
