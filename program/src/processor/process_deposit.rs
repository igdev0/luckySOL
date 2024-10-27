use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
};

pub fn process_deposit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
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
