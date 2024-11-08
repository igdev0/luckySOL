mod helpers;
use borsh::BorshDeserialize;
use helpers::initialize_stake_pool_tx;

use solana_lottery_program::{
    processor::{
        find_player_pda_account, find_player_token_pda_account, find_stake_pool_mint_pda,
        find_stake_pool_vault_pda,
    },
    state::{DraftWinner, Instruction as LotoInstruction, PoolStorageData, TicketAccountData},
};
use solana_program_test::*;
use solana_sdk::{instruction::AccountMeta, program_pack::Pack, signer::Signer};

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

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
            total_tickets: 1,
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
            total_tickets: 2,
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

    let player_token_account = client
        .get_account(player_token_pda_address)
        .await
        .unwrap()
        .unwrap();

    let unpacked = spl_token_2022::state::Account::unpack(&player_token_account.data).unwrap();

    assert_eq!(unpacked.amount, 2);
}

#[tokio::test]
async fn can_select_winners_and_widthdraw_prize() {
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

    let player_pda_account = find_player_pda_account(&solana_lottery_program::ID, &player.pubkey());
    let player_token_pda_account =
        find_player_token_pda_account(&solana_lottery_program::ID, &player.pubkey());

    let tickets = vec!["0", "1", "3", "6", "2", "6"];
    let ticket_hashes: Vec<[u8; 32]> = tickets.iter().map(|t| Sha256::hash(t.as_bytes())).collect();
    let indices_to_prove = vec![4, 5]; // two winning tickets
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&ticket_hashes);
    let merkle_root = merkle_tree.root().expect("unable to get the root");

    let tx = helpers::purchase_ticket_tx(
        &solana_lottery_program::ID,
        &pool_authority,
        &player,
        player_pda_account.0,
        player_token_pda_account.0,
        pool_vault_account,
        pool_mint_account,
        recent_blockhash,
        &LotoInstruction::PurchaseTicket(TicketAccountData {
            merkle_root,
            total_tickets: tickets.len() as u64,
        }),
    );

    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let player_token_account = client
        .get_account(player_pda_account.0)
        .await
        .unwrap()
        .unwrap();
    let previous_lamports = player_token_account.lamports;

    // Process the winners

    let proof = merkle_tree.proof(&indices_to_prove);
    let proof_bytes = proof.to_bytes();

    let winners_instruction_data = vec![DraftWinner {
        amount: 100_000_000,
        token_account: player_token_pda_account.0,
        address: player_pda_account.0,
        tickets: indices_to_prove.iter().map(|&i| ticket_hashes[i]).collect(),
        proof: proof_bytes,
        ticket_indices: indices_to_prove,
    }];

    let winner_accounts = vec![
        AccountMeta::new(player_pda_account.0, false),
        AccountMeta::new(player_token_pda_account.0, false),
        AccountMeta::new(spl_token_2022::ID, false),
    ];

    let tx = helpers::process_winners_tx(
        &pool_authority,
        winners_instruction_data,
        winner_accounts,
        recent_blockhash,
    );

    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let player_token_account = client
        .get_account(player_pda_account.0)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        player_token_account.lamports,
        100_000_000 + previous_lamports
    );

    let pool_vault_account = client
        .get_account(pool_vault_account)
        .await
        .unwrap()
        .unwrap();

    let PoolStorageData { draft_count, .. } =
        PoolStorageData::deserialize(&mut pool_vault_account.data.as_slice()).unwrap();

    assert_eq!(draft_count, 1);

    let player_total_lamports = client.get_balance(player.pubkey()).await.unwrap();
    let player_token_account = client
        .get_account(player_token_pda_account.0)
        .await
        .unwrap()
        .unwrap();

    let player_token_account_unpacked =
        spl_token_2022::state::Account::unpack(&player_token_account.data).unwrap();

    assert_eq!(player_token_account_unpacked.amount, 0);

    let tx =
        helpers::process_withdraw_tx(&player, player_pda_account.0, 100_000_000, recent_blockhash);

    let tx_cost = client
        .get_fee_for_message(tx.message.clone())
        .await
        .unwrap()
        .unwrap();
    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .unwrap();

    assert_eq!(
        client.get_balance(player.pubkey()).await.unwrap(),
        100_000_000 + (player_total_lamports - tx_cost)
    );

    let tx = helpers::close_account_tx(
        &player,
        player_pda_account.0,
        player_token_pda_account.0,
        &pool_authority.pubkey(),
        &pool_mint_account,
        recent_blockhash,
    );
    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .unwrap();

    let updated_player_pda_account = client.get_account(player_pda_account.0).await.unwrap();
    assert_eq!(updated_player_pda_account, None);
    let updated_player_pda_token_account = client
        .get_account(player_token_pda_account.0)
        .await
        .unwrap();

    assert_eq!(updated_player_pda_token_account, None);
}
