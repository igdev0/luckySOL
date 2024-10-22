use crate::{
    error::LotteryError,
    state::{LotoInstruction, PoolStorageAccount, PoolStorageSeed, TicketAccountData},
};
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

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
    let stake_pool_authority = next_account_info(&mut accounts)?;

    // The PDA vault of the stake pool which will be used to store the tickets.
    let stake_pool_vault = next_account_info(&mut accounts)?;

    // The receipt mint is the mint which will be used to create new tokens as users purchase tickets.
    let receipt_mint = next_account_info(&mut accounts)?;

    // The receipt mint authority is the authority which can mint new receipt tokens.
    let receipt_mint_authority = next_account_info(&mut accounts)?;

    // The system program account
    let system_program_account = next_account_info(&mut accounts)?;

    let (pool_vault_addr, bump) = Pubkey::find_program_address(
        &[
            PoolStorageSeed::StakePool.as_bytes(),
            stake_pool_authority.key.as_ref(),
        ],
        program_id,
    );

    let rent = Rent::get()?;
    let account_size = std::mem::size_of::<PoolStorageAccount>();

    let exempt_balance = rent.minimum_balance(account_size);

    if stake_pool_authority.lamports() < (exempt_balance + amount) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }
    msg!("Initialize stake pool account");
    // Initialize stake pool account
    let stake_pool_instr = system_instruction::create_account(
        &stake_pool_authority.key,
        &pool_vault_addr,
        exempt_balance + amount,
        account_size as u64,
        program_id,
    );

    invoke_signed(
        &stake_pool_instr,
        &[
            stake_pool_authority.clone(),
            stake_pool_vault.clone(),
            system_program_account.clone(),
        ],
        &[&[
            PoolStorageSeed::StakePool.as_bytes(),
            &stake_pool_authority.key.as_ref(),
            &[bump],
        ]],
    )?;

    // Init the storage account for the stake pool

    msg!("Initialize stake pool storage account");

    let mut stake_pool_data = stake_pool_vault.try_borrow_mut_data()?;

    let data = PoolStorageAccount {
        receipt_mint: receipt_mint.key.clone(),
        receipt_mint_authority: receipt_mint_authority.key.clone(),
        stake_pool_authority: stake_pool_authority.key.clone(),
    };

    data.serialize(&mut *stake_pool_data)?;
    Ok(())
}

fn process_player_initialization(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_data: TicketAccountData,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();
    // Account payer
    let payer = next_account_info(&mut accounts)?;
    // Account PDA for payer
    let vault_pda = next_account_info(&mut accounts)?;
    // Stake pool vault
    let stake_pool_vault = next_account_info(&mut accounts)?;
    // Rent exempt check
    let rent = Rent::get()?;
    let space = std::mem::size_of::<TicketAccountData>();

    if !rent.is_exempt(payer.lamports(), space) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    if !payer.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    if !vault_pda.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    }

    if stake_pool_vault.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::UninitializedAccount);
    }

    let seeds = &[payer.key.as_ref()];
    let (pda, bump_seed) = Pubkey::find_program_address(seeds, program_id);

    let lamports = rent.minimum_balance(space);

    let instruction =
        system_instruction::create_account(&payer.key, &pda, lamports, space as u64, program_id);

    invoke_signed(
        &instruction,
        &[payer.clone(), vault_pda.clone()],
        &[&[payer.key.as_ref(), &[bump_seed]]],
    )?;

    let stake_pool_vault_data = stake_pool_vault.try_borrow_data()?;
    let stake_pool_vault_data = PoolStorageAccount::try_from_slice(&stake_pool_vault_data)?;

    let token_account_instruction = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &payer.key,
        &stake_pool_vault_data.receipt_mint,
        &pda,
    )?;

    invoke_signed(
        &token_account_instruction,
        &[payer.clone()],
        &[&[payer.key.as_ref(), &[bump_seed]]],
    )?;

    let serialized_data = to_vec(&account_data).unwrap();

    vault_pda.try_borrow_mut_data()?[..serialized_data.len()].copy_from_slice(&serialized_data);

    Ok(())
}
