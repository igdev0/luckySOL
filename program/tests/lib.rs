mod helpers;

use helpers::initialize_stake_pool_tx;
use solana_lottery_program::{
    processor::find_stake_pool_mint_pda,
    state::{LotoInstruction, PoolStorageSeed},
};
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    signer::Signer,
    system_program,
    sysvar::{self},
    transaction::Transaction,
};

const PROGRAM_ID: Pubkey = solana_lottery_program::ID;

#[tokio::test]
async fn initialize_pool() {
    let program_id: Pubkey = PROGRAM_ID;
    let (mut client, pool_authority, recent_blockhash) = helpers::setup().await;

    let (pool_mint_account, ..) = find_stake_pool_mint_pda(&program_id, &pool_authority.pubkey());
    let tx = initialize_stake_pool_tx(program_id, &pool_authority, recent_blockhash);
    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let mint_account = client
        .get_account(pool_mint_account)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(mint_account.owner, spl_token_2022::ID);

    let unpacked = spl_token_2022::state::Mint::unpack(&mint_account.data).unwrap();

    assert_eq!(unpacked.supply, 0);
    assert_eq!(unpacked.decimals, 0);
}

#[tokio::test]
async fn ticket_purchase() {}
