use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult,
    program::invoke_signed, pubkey::Pubkey,
};

use solana_program::program_pack::Pack;

use spl_token_2022::state::Account;

use crate::{error::LotteryError, state::PoolStorageSeed};

use super::find_stake_pool_mint_pda;

pub fn process_close_player_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts = &mut accounts.iter();
    let player_account = next_account_info(accounts)?;
    let player_pda_account = next_account_info(accounts)?;
    let player_token_pda_account = next_account_info(accounts)?;
    let pool_authority = next_account_info(accounts)?;
    let mint_account = next_account_info(accounts)?;

    if !player_account.is_signer {
        return Err(LotteryError::InvalidSigner.into());
    }

    if player_pda_account.data_is_empty() {
        return Err(LotteryError::InvalidAccount.into());
    }

    if player_token_pda_account.data_is_empty() {
        return Err(LotteryError::InvalidAccount.into());
    }

    let token_account_unpacked = Account::unpack(&player_token_pda_account.try_borrow_data()?)?;

    // Close the token_account if its amount is 0
    if token_account_unpacked.amount == 0 {
        let ix = spl_token_2022::instruction::close_account(
            &spl_token_2022::ID,
            player_token_pda_account.key,
            player_account.key,
            &mint_account.key,
            &[],
        )?;

        let auth_bump = find_stake_pool_mint_pda(program_id, pool_authority.key).1;

        invoke_signed(
            &ix,
            &[
                mint_account.clone(),
                player_token_pda_account.clone(),
                player_account.clone(),
            ],
            &[&[
                PoolStorageSeed::ReceiptMint.as_bytes(),
                pool_authority.key.as_ref(),
                &[auth_bump],
            ]],
        )?;
    }

    let mut player_account_lamports = player_account.try_borrow_mut_lamports()?;

    **player_account_lamports += player_pda_account.lamports();
    **player_pda_account.try_borrow_mut_lamports()? = 0;

    player_pda_account.assign(&program_id);
    player_pda_account.realloc(0, false)?;
    Ok(())
}
