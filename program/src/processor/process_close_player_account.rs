use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult,
    program::invoke, pubkey::Pubkey,
};

use crate::error::LotteryError;

pub fn process_close_player_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts = &mut accounts.iter();
    let player_account = next_account_info(accounts)?;
    let player_pda_account = next_account_info(accounts)?;
    let player_token_pda_account = next_account_info(accounts)?;
    let pool_authority = next_account_info(accounts)?;

    if !player_account.is_signer {
        return Err(LotteryError::InvalidSigner.into());
    }

    if player_pda_account.data_is_empty() {
        return Err(LotteryError::InvalidAccount.into());
    }

    if player_token_pda_account.data_is_empty() {
        return Err(LotteryError::InvalidAccount.into());
    }

    let ix = spl_token_2022::instruction::close_account(
        &spl_token_2022::ID,
        player_token_pda_account.key,
        player_account.key,
        &pool_authority.key,
        &[player_account.key],
    )?;

    invoke(
        &ix,
        &[
            player_account.clone(),
            player_token_pda_account.clone(),
            pool_authority.clone(),
        ],
    )?;

    let mut player_account_lamports = player_account.try_borrow_mut_lamports()?;

    **player_account_lamports += player_pda_account.lamports();
    **player_pda_account.try_borrow_mut_lamports()? = 0;

    // **player_account_lamports += player_token_pda_account.lamports();
    // **player_token_pda_account.try_borrow_mut_lamports()? = 0;

    player_pda_account.assign(&program_id);
    player_pda_account.realloc(0, false)?;
    Ok(())
}
