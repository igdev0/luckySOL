mod helpers;

use borsh::BorshDeserialize;
use helpers::initialize_stake_pool_tx;
use solana_lottery_program::{
    processor::{
        find_player_pda_account, find_player_token_pda_account, find_stake_pool_mint_pda,
        find_stake_pool_vault_pda,
    },
    state::LotoInstruction,
};
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    signer::Signer,
    system_program,
    sysvar::{self},
    transaction::Transaction,
};

#[tokio::test]
async fn initialize_pool() {
    let (mut client, pool_authority, recent_blockhash, ..) = helpers::setup().await;

    let (pool_mint_account, ..) =
        find_stake_pool_mint_pda(&solana_lottery_program::ID, &pool_authority.pubkey());
    let tx = initialize_stake_pool_tx(
        &solana_lottery_program::ID,
        &pool_authority,
        &recent_blockhash,
    );
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
async fn ticket_purchase() {
    let (mut client, pool_authority, recent_blockhash, player) = helpers::setup().await;

    let (pool_mint_account, ..) =
        find_stake_pool_mint_pda(&solana_lottery_program::ID, &pool_authority.pubkey());
    let (pool_vault_account, ..) =
        find_stake_pool_vault_pda(&solana_lottery_program::ID, &pool_authority.pubkey());
    let tx = initialize_stake_pool_tx(
        &solana_lottery_program::ID,
        &pool_authority,
        &recent_blockhash,
    );

    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let ticket_data =
        LotoInstruction::PurchaseTicket(solana_lottery_program::state::TicketAccountData {
            merkle_root: [0; 32],
            address: player.pubkey(),
        });
    let (player_pda_address, ..) =
        find_player_pda_account(&solana_lottery_program::ID, &player.pubkey());

    let (player_token_pda_address, ..) =
        find_player_token_pda_account(&solana_lottery_program::ID, &player.pubkey());

    let tx = helpers::purchase_ticket_tx(
        &solana_lottery_program::ID,
        &pool_authority,
        &player,
        player_pda_address,
        player_token_pda_address,
        pool_vault_account,
        pool_mint_account,
        recent_blockhash,
        &ticket_data,
    );

    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let ticket_account = client
        .get_account(player_pda_address)
        .await
        .unwrap()
        .unwrap();

    let unpacked =
        solana_lottery_program::state::TicketAccountData::try_from_slice(&ticket_account.data)
            .unwrap();

    assert_eq!(unpacked.merkle_root, [0; 32]);
    assert_eq!(unpacked.address, player.pubkey());

    let player_token_account = client
        .get_account(player_token_pda_address)
        .await
        .unwrap()
        .unwrap();

    let unpacked = spl_token_2022::state::Account::unpack(&player_token_account.data).unwrap();

    assert_eq!(unpacked.amount, 1);

    // Purchase second ticket

    let ticket_data =
        LotoInstruction::PurchaseTicket(solana_lottery_program::state::TicketAccountData {
            merkle_root: [1; 32],
            address: player.pubkey(),
        });
    let new_transaction = helpers::purchase_ticket_tx(
        &solana_lottery_program::ID,
        &pool_authority,
        &player,
        player_pda_address,
        player_token_pda_address,
        pool_vault_account,
        pool_mint_account,
        recent_blockhash,
        &ticket_data,
    );

    client
        .process_transaction_with_commitment(
            new_transaction,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let ticket_account = client
        .get_account(player_pda_address)
        .await
        .unwrap()
        .unwrap();

    let unpacked =
        solana_lottery_program::state::TicketAccountData::try_from_slice(&ticket_account.data)
            .unwrap();

    dbg!(&unpacked);
    assert_eq!(unpacked.merkle_root, [1; 32]);
    assert_eq!(unpacked.address, player.pubkey());

    let player_token_account = client
        .get_account(player_token_pda_address)
        .await
        .unwrap()
        .unwrap();

    let unpacked = spl_token_2022::state::Account::unpack(&player_token_account.data).unwrap();

    assert_eq!(unpacked.amount, 2);
}
