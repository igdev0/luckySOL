use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::state::{AccountData, LotoInstruction};

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
        LotoInstruction::CloseAccount => Ok(()),
        LotoInstruction::PurchaseTickets { guesses: _ } => Ok(()),
        LotoInstruction::SelectWinners(_winners) => Ok(()),
    }
}

fn process_initialization(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_data: AccountData,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();
    // Account payer
    let payer = next_account_info(&mut accounts)?;
    // Account PDA for payer
    let vault_pda = next_account_info(&mut accounts)?;
    // Rent exempt check
    let rent = Rent::get()?;
    let space = size_of::<AccountData>();

    if !rent.is_exempt(payer.lamports(), space) {
        return Err(solana_program::program_error::ProgramError::AccountNotRentExempt);
    }

    if !payer.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    if vault_pda.owner != program_id {
        return Err(solana_program::program_error::ProgramError::InvalidAccountOwner);
    }

    if !vault_pda.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    }

    let seeds = payer.key.as_ref();
    let (pda, bump_seed) = Pubkey::find_program_address(&[seeds], program_id);

    let lamports = rent.minimum_balance(space);

    let instruction =
        system_instruction::create_account(&payer.key, &pda, lamports, space as u64, program_id);

    invoke_signed(
        &instruction,
        &[payer.clone(), vault_pda.clone()],
        &[&[payer.key.as_ref(), &[bump_seed]]],
    )?;

    let mut data = vault_pda.data.as_ref().borrow_mut();
    // Initialize the data from the instruction_data, so the user can directly add guesses, no unnecessry two transactions

    account_data.serialize(&mut *data)?;
    Ok(())
}
