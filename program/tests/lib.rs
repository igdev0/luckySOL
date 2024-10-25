mod helpers;

use borsh::BorshDeserialize;
use helpers::initialize_stake_pool_tx;
use solana_lottery_program::{
    processor::{
        find_player_pda_account, find_player_token_pda_account, find_stake_pool_mint_pda,
        find_stake_pool_vault_pda,
    },
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

    let accounts = vec![
        AccountMeta::new(player.pubkey(), true),
        AccountMeta::new(player_pda_address, false),
        AccountMeta::new(player_token_pda_address, false),
        AccountMeta::new(pool_vault_account, false),
        AccountMeta::new(pool_mint_account, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(solana_lottery_program::id(), false),
    ];

    let instruction =
        Instruction::new_with_borsh(solana_lottery_program::ID, &ticket_data, accounts);

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&player.pubkey()),
        &[&player],
        recent_blockhash,
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
    assert_eq!(unpacked.address, player_pda_address);
}
