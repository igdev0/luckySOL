use crate::{
    error::LotteryError,
    state::{LotoInstruction, TicketAccountData},
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
        LotoInstruction::Initialize(account_data) => {
            process_initialization(program_id, accounts, account_data)
        }
        LotoInstruction::CloseAccount => Err(LotteryError::NotImplemented.into()),
        LotoInstruction::PurchaseTicket { ticket: _ } => Err(LotteryError::NotImplemented.into()),
        LotoInstruction::SelectWinners(_winners) => Err(LotteryError::NotImplemented.into()),
    }
}

fn process_initialization(
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
