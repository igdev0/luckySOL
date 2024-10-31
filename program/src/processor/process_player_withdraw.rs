use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub fn process_player_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let mut accounts = accounts.iter();

    let player_account = next_account_info(&mut accounts)?;

    let player_pda_account = next_account_info(&mut accounts)?;

    if !player_account.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    if amount > **player_pda_account.try_borrow_lamports()? {
        return Err(solana_program::program_error::ProgramError::InsufficientFunds);
    }

    **player_pda_account.try_borrow_mut_lamports()? -= amount;

    **player_account.try_borrow_mut_lamports()? += amount;

    Ok(())
}
