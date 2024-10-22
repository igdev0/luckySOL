use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use std::str::FromStr;

use solana_lottery_program::{
    processor::processor as lottery_processor,
    state::{LotoInstruction, PoolStorageAccount, PoolStorageSeed, TicketAccountData},
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    sysvar::{self},
    transaction::Transaction,
};

const RECEIPT_MINT: &str = "Au4Ltjubh7rSdbGqKoaPsAno6aSmggaqwBwHRP2jJEFD";
const RECEIPT_MINT_AUTHORITY: &str = "68HumYhXUHYoUXtzbXbv6FQ9WcMMQmESJCr1zw27a9xn";

#[tokio::test]
async fn initialize_pool() {
    let program_id = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "solana_lottery_program",
        program_id,
        processor!(lottery_processor),
    );

    let (mut client, pool_authority, recent_blockhash) = program_test.start().await;

    let acc = client.get_account(pool_authority.pubkey()).await.unwrap();

    dbg!(acc);
    let receipt_mint = Pubkey::from_str(&RECEIPT_MINT).unwrap();
    let receipt_mint_authority = Pubkey::from_str(&RECEIPT_MINT_AUTHORITY).unwrap();
    let (pool_storage_account_address, _bump) = Pubkey::find_program_address(
        &[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority.pubkey().to_bytes(),
        ],
        &program_id,
    );

    let instruction_data = LotoInstruction::InitializePool(100_000_000);

    let accounts = vec![
        AccountMeta::new_readonly(pool_authority.pubkey(), true),
        AccountMeta::new(pool_storage_account_address, false),
        AccountMeta::new_readonly(receipt_mint, false),
        AccountMeta::new_readonly(receipt_mint_authority, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];

    let instruction = Instruction::new_with_borsh(program_id, &instruction_data, accounts);

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&pool_authority.pubkey()),
        &[&pool_authority],
        recent_blockhash,
    );

    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");
}

#[tokio::test]
#[ignore = "Not ready yet"]
async fn initialize_player_account() {
    let program_id = Pubkey::new_unique();
    let player_key = Keypair::new();
    let token_mint = Pubkey::from_str(RECEIPT_MINT).expect("unable to parse mint");
    let token_mint_authority =
        Pubkey::from_str(RECEIPT_MINT_AUTHORITY).expect("Unable to parse mint authority");
    let mut program_test = ProgramTest::new(
        "solana_lottery_program",
        program_id,
        processor!(lottery_processor),
    );

    let player_account = Account::new(100_000_000_000_000, 0, &system_program::ID);
    program_test.add_account(player_key.pubkey(), player_account);

    let (mut client, .., recent_blockhash) = program_test.start().await;

    let tickets = ["0", "1", "2", "3", "4"];
    let leaves: Vec<[u8; 32]> = tickets
        .iter()
        .map(|ticket| Sha256::hash(ticket.as_bytes()))
        .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let merkle_root = merkle_tree
        .root()
        .ok_or("There was some error while processing merkle root")
        .expect("Wasn't able to get the root of the merkle tree");

    let instruction_data = LotoInstruction::PurchaseTicket(TicketAccountData {
        address: player_key.pubkey(),
        merkle_root,
    });

    let (player_vault, ..) =
        Pubkey::find_program_address(&[player_key.pubkey().as_ref()], &program_id);

    let accounts = vec![
        AccountMeta::new(player_key.pubkey(), true),
        AccountMeta::new(player_vault, false),
        AccountMeta::new(sysvar::rent::ID, false),
        AccountMeta::new(system_program::ID, false),
    ];

    let instruction = Instruction::new_with_borsh(program_id, &instruction_data, accounts);

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&player_key.pubkey()),
        &[&player_key],
        recent_blockhash,
    );
    client
        .process_transaction_with_commitment(
            tx,
            solana_sdk::commitment_config::CommitmentLevel::Finalized,
        )
        .await
        .expect("Unable to process data");

    let account_data = client
        .get_account_data_with_borsh::<TicketAccountData>(player_vault)
        .await
        .expect(
            "Error while trying to get the account data or parsing the account data with borsh",
        );

    dbg!(&account_data);
    assert_eq!(account_data.merkle_root, merkle_root);
}
