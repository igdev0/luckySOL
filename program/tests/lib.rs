use solana_lottery_program::{
    processor::processor as lottery_processor,
    state::{DraftNumbers, LotoInstruction, TicketAccountData},
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

#[tokio::test]
async fn initialize_player_account() {
    let program_id = Pubkey::new_unique();
    let player_key = Keypair::new();

    let mut program_test = ProgramTest::new(
        "solana_lottery_program",
        program_id,
        processor!(lottery_processor),
    );

    let player_account = Account::new(100_000_000_000_000, 0, &system_program::ID);
    program_test.add_account(player_key.pubkey(), player_account);

    let (mut client, .., recent_blockhash) = program_test.start().await;

    let player_balance = client.get_balance(player_key.pubkey()).await.unwrap();
    assert_eq!(player_balance, 100_000_000_000_000);
    let guess: DraftNumbers = [10, 20, 40, 34, 20, 12, 47];
    let ticket = [guess, [0; 7], [0; 7], [0; 7]];
    let instruction_data = LotoInstruction::Initialize(TicketAccountData {
        ticket,
        account_index: 0,
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
    assert_eq!(account_data.ticket, ticket);
    // assert!(transaction_status.is_ok());
}
