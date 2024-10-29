use borsh::BorshDeserialize;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::LotteryError,
    state::{TicketAccountData, Winner},
};

use super::find_stake_pool_vault_pda;

fn process_winner<'a>(
    stake_pool_account: &AccountInfo<'a>,
    account: &AccountInfo<'a>,
    amount: u64,
    tickets: Vec<[u8; 32]>,
    proof: Vec<u8>,
    ticket_indices: Vec<usize>,
) -> ProgramResult {
    let account_data = account.try_borrow_data()?;
    let account_data = TicketAccountData::try_from_slice(&account_data)?;

    let proof = MerkleProof::<Sha256>::try_from(proof).unwrap();
    if proof.verify(
        account_data.merkle_root,
        &ticket_indices,
        &tickets,
        account_data.total_tickets as usize,
    ) {
        // Transfer the amount to the winner
        stake_pool_account
            .try_borrow_mut_lamports()?
            .checked_sub(amount)
            .unwrap();

        account
            .try_borrow_mut_lamports()?
            .checked_add(amount)
            .unwrap();

        // Burn the receipt token
    }

    Ok(())
}

pub fn process_winners(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    winners: Vec<Winner>,
) -> ProgramResult {
    let mut accounts = accounts.iter();

    let authority_account = next_account_info(&mut accounts)?;

    let stake_pool_account = next_account_info(&mut accounts)?;

    // Verify that the authority is the signer of the transaction
    if !authority_account.is_signer {
        return Err(LotteryError::AuthorityMustSign.into());
    }

    // Verify that the stake pool account is owned by the program
    if stake_pool_account.owner != program_id {
        return Err(LotteryError::IncorrectOwner.into());
    }
    let (stake_pool_pda, ..) = find_stake_pool_vault_pda(program_id, authority_account.key);

    // Verify that the stake pool account is the correct one
    if stake_pool_pda != *stake_pool_account.key {
        return Err(LotteryError::InvalidStakePoolVault.into());
    }

    accounts
        .map(|account| -> ProgramResult {
            let winner = winners
                .iter()
                .find(|winner| winner.address == *account.key)
                .unwrap();

            process_winner(
                stake_pool_account,
                account,
                winner.amount,
                winner.tickets.clone(),
                winner.proof.clone(),
            )
        })
        .collect()
}
