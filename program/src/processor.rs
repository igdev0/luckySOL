use crate::{
    error::LotteryError,
    state::{LotoInstruction, PoolStorageAccount, PoolStorageSeed, TicketAccountData},
};
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
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

    // The PDA vault of the stake pool
    let stake_pool_vault = next_account_info(&mut accounts)?;

    // This is the actual receipt mint address
    let receipt_mint = next_account_info(&mut accounts)?;

    // We will use the mint authority to create new tokens as user purchases tickets.
    // 1 receipt token per ticket
    let receipt_mint_authority = next_account_info(&mut accounts)?;

    // We will need receipt mint owner when the user decides to close acccount.
    let receipt_mint_owner = next_account_info(&mut accounts)?;

    let system_program_account = next_account_info(&mut accounts)?;

    let (pool_vault_addr, bump) =
        Pubkey::find_program_address(&[PoolStorageSeed::StakePool.as_bytes()], program_id);

    let rent = Rent::get()?;
    let account_size = std::mem::size_of::<PoolStorageAccount>();

    let exempt_balance = rent.minimum_balance(account_size);

    if stake_pool_authority.lamports() < (exempt_balance + amount) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    // Initialize stake pool account
    let stake_pool_instr = system_instruction::create_account(
        program_id,
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
        &[&[PoolStorageSeed::StakePool.as_bytes(), &[bump]]],
    )?;

    // Init the storage account for the stake pool

    let mut stake_pool_data = stake_pool_vault.try_borrow_mut_data()?;

    let data = PoolStorageAccount {
        receipt_mint: receipt_mint.key.clone(),
        receipt_mint_authority: receipt_mint_authority.key.clone(),
        receipt_mint_owner: receipt_mint_owner.key.clone(),
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
    let serialized_data = to_vec(&account_data).unwrap();

    vault_pda.try_borrow_mut_data()?[..serialized_data.len()].copy_from_slice(&serialized_data);

    Ok(())
}
