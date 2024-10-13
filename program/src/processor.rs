use crate::{
    error::LotteryError,
    state::{LotoInstruction, PoolAccount, TicketAccountData},
};
use borsh::{to_vec, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn processor(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instr = LotoInstruction::try_from_slice(instruction_data)?;
    match instr {
        LotoInstruction::InitializePool => process_pool_initialization(program_id, accounts),
        LotoInstruction::PurchaseTicket(account_data) => {
            process_player_initialization(program_id, accounts, account_data)
        }
        LotoInstruction::ClosePlayerAccount => Err(LotteryError::NotImplemented.into()),
        LotoInstruction::SelectWinnersAndAirdrop() => Err(LotteryError::NotImplemented.into()),
    }
}

fn process_pool_initialization(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut accounts = accounts.into_iter();
    let payer = next_account_info(&mut accounts)?;
    let pool_pda_account = next_account_info(&mut accounts)?;
    let system_program_account = next_account_info(&mut accounts)?;
    let (vault, bump) = Pubkey::find_program_address(&[b"pool"], program_id);

    let rent = Rent::get()?;
    let account_size = std::mem::size_of::<PoolAccount>();
    if !rent.is_exempt(payer.lamports(), std::mem::size_of::<PoolAccount>()) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    let ballance = rent.minimum_balance(account_size);

    let instr = system_instruction::create_account(
        program_id,
        &vault,
        ballance,
        account_size as u64,
        program_id,
    );

    invoke_signed(
        &instr,
        &[
            payer.clone(),
            pool_pda_account.clone(),
            system_program_account.clone(),
        ],
        &[&[b"pool", &[bump]]],
    )?;
    // @todo: process account creation and initialization afterwards.

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
