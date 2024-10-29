use borsh::BorshDeserialize;
use rs_merkle::{algorithms::Sha256, MerkleProof};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
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
    // let account_data = account.try_borrow_data()?;
    let account_data = TicketAccountData::try_from_slice(&account.data.borrow())?;
    msg!("Account data: {:?}", account_data);
    msg!("Total before in the winner account: {}", account.lamports());
    let proof = MerkleProof::<Sha256>::try_from(proof).expect("Provided invalid proof");
    if proof.verify(
        account_data.merkle_root,
        &ticket_indices,
        &tickets,
        account_data.total_tickets as usize,
    ) {
        if amount > **stake_pool_account.try_borrow_lamports()? {
            return Err(LotteryError::InsufficientFunds.into());
        }

        // Transfer the amount to the winner
        **stake_pool_account.try_borrow_mut_lamports()? -= amount;
        **account.try_borrow_mut_lamports()? += amount;

        // Burn the receipt token
    } else {
        return Err(LotteryError::InvalidTicket.into());
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

    winners
        .iter()
        .map(|winner| -> ProgramResult {
            let account_info = accounts
                .find(|account| account.key == &winner.address)
                .expect("Unable to find the account in the list");
            process_winner(
                stake_pool_account,
                account_info,
                winner.amount,
                winner.tickets.clone(),
                winner.proof.clone(),
                winner.ticket_indices.clone(),
            )
        })
        .collect()
}
