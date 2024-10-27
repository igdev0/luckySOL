use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub fn process_winners(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    winners: Vec<Pubkey>,
) -> ProgramResult {
    Ok(())
}
