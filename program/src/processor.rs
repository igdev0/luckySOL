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
            process_player_initialization(program_id, accounts, account_data)
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

fn process_pool_initialization(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();

    // The stake pool authority, is the authority which can verify tickets if they are valid and proceeding to airdrop prises.
    let pool_authority = next_account_info(&mut accounts)?;

    // The PDA vault of the stake pool which will be used to store the tickets.
    let stake_pool_vault = next_account_info(&mut accounts)?;

    // The receipt mint is the mint which will be used to create new tokens as users purchase tickets.
    let receipt_mint = next_account_info(&mut accounts)?;

    // The system program account
    let _system_program_account = next_account_info(&mut accounts)?;
    let rent_account = next_account_info(&mut accounts)?;
    let spl_token_2022_account = next_account_info(&mut accounts)?;

    let (pool_vault_addr, bump) = Pubkey::find_program_address(
        &[
            PoolStorageSeed::StakePool.as_bytes(),
            pool_authority.key.as_ref(),
        ],
        program_id,
    );

    let rent = Rent::get()?;
    let account_size = std::mem::size_of::<PoolStorageAccount>();

    let exempt_balance = rent.minimum_balance(account_size);

    if pool_authority.lamports() < (exempt_balance + amount) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    let token_account_exempt_balance = rent.minimum_balance(spl_token_2022::state::Mint::LEN);
    // Initialize stake pool account
    let token_account_instr = system_instruction::create_account(
        &pool_authority.key,
        &stake_pool_vault.key,
        token_account_exempt_balance,
        spl_token_2022::state::Mint::LEN as u64,
        &spl_token_2022::ID,
    );

    invoke_signed(
        &token_account_instr,
        &[pool_authority.clone(), stake_pool_vault.clone()],
        &[&[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority.key.as_ref(),
            &[bump],
        ]],
    )?;

    let token_init_instruction = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::ID,
        &stake_pool_vault.key,
        &stake_pool_vault.key,
        None,
        0,
    )?;

    msg!("Creating SPL 2022 mint");
    invoke_signed(
        &token_init_instruction,
        &[
            pool_authority.clone(),
            receipt_mint.clone(),
            stake_pool_vault.clone(),
            spl_token_2022_account.clone(),
            rent_account.clone(),
        ],
        &[&[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority.key.as_ref(),
            &[bump],
        ]],
    )?;

    let mut stake_pool_data = stake_pool_vault.try_borrow_mut_data()?;

    let data = PoolStorageAccount {
        receipt_mint: receipt_mint.key.clone(),
    };

    // let spl_instruction = spl_token_2022::instruction::set_authority(&spl_token_2022::ID, owned_pubkey, new_authority_pubkey, authority_type, owner_pubkey, signer_pubkeys)?;

    data.serialize(&mut *stake_pool_data)?;
    Ok(())
}

/// Process the player initialization
/// This function will create a new account for the player and transfer the ticket price to the stake pool vault.
/// The player account will be initialized with the ticket data.
/// The player account will be owned by the program.
fn process_player_initialization(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_data: TicketAccountData,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();
    // Account payer
    let player = next_account_info(&mut accounts)?;
    // Account PDA for payer
    let vault_pda = next_account_info(&mut accounts)?;
    // Stake pool vault
    let stake_pool_vault = next_account_info(&mut accounts)?;
    // Rent exempt check
    let rent = Rent::get()?;
    let space = std::mem::size_of::<TicketAccountData>();
    if !rent.is_exempt(player.lamports(), space) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    if !player.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    if !vault_pda.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    }

    if stake_pool_vault.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::UninitializedAccount);
    }

    let seeds = &[player.key.as_ref()];
    let (pda, bump_seed) = Pubkey::find_program_address(seeds, program_id);

    let lamports = rent.minimum_balance(space);

    let instruction =
        system_instruction::create_account(&player.key, &pda, lamports, space as u64, program_id);

    invoke_signed(
        &instruction,
        &[player.clone(), vault_pda.clone()],
        &[&[player.key.as_ref(), &[bump_seed]]],
    )?;

    let stake_pool_vault_data = stake_pool_vault.try_borrow_data()?;
    let stake_pool_vault_data = PoolStorageAccount::try_from_slice(&stake_pool_vault_data)?;

    let token_account_instruction = spl_token_2022::instruction::initialize_account(
        &spl_token_2022::ID,
        &player.key,
        &stake_pool_vault_data.receipt_mint,
        &pda,
    )?;

    let ticket_purchase_instr =
        system_instruction::transfer(player.key, stake_pool_vault.key, TICKET_PRICE);

    invoke_signed(
        &ticket_purchase_instr,
        &[player.clone(), stake_pool_vault.clone()],
        &[&[player.key.as_ref(), &[bump_seed]]],
    )?;

    invoke_signed(
        &token_account_instruction,
        &[player.clone()],
        &[&[player.key.as_ref(), &[bump_seed]]],
    )?;

    // Now send recipe token back to the player

    let token_transfer_instruction = spl_token_2022::instruction::mint_to(
        &spl_token_2022::ID,
        &stake_pool_vault_data.receipt_mint,
        &pda,
        &player.key,
        &[],
        1,
    )?;

    // Serialize the account data and store it in the account

    let serialized_data = to_vec(&account_data).unwrap();

    vault_pda.try_borrow_mut_data()?[..serialized_data.len()].copy_from_slice(&serialized_data);

    Ok(())
}
