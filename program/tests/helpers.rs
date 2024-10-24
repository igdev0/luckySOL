use std::ops::Deref;

use solana_lottery_program::ID;
use solana_program_test::ProgramTest;
use solana_program_test::*;
use solana_sdk::{
    commitment_config::CommitmentLevel, program_error::ProgramError, program_pack::Pack,
    pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};

type Res<T> = Result<T, ProgramError>;

pub async fn setup() -> (BanksClient, Keypair, solana_sdk::hash::Hash) {
    let program = ProgramTest::new(
        "solana_lottery_program",
        ID,
        processor!(solana_lottery_program::processor::processor),
    );
    solana_logger::setup_with_default("solana=trace");
    let (mut banks_client, payer, recent_blockhash) = program.start().await;

    (banks_client, payer, recent_blockhash)
}

pub async fn init_receipt_token(
    client: &mut BanksClient,
    mint_authority: &Keypair,
    recent_blockhash: solana_sdk::hash::Hash,
) -> Res<Keypair> {
    let token_keypair = Keypair::new();
    // 2. Create the account for the mint and fund it with rent-exempt balance

    let rent = client.get_rent().await.unwrap();

    let mint_rent = rent.minimum_balance(spl_token_2022::state::Mint::LEN as usize);

    // 2. Create the account for the mint and fund it with rent-exempt balance
    let create_account_instruction = solana_sdk::system_instruction::create_account(
        &mint_authority.pubkey(),
        &token_keypair.pubkey(),
        mint_rent,
        spl_token_2022::state::Mint::LEN as u64,
        &spl_token_2022::ID,
    );

    let create_token_instruction = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::ID,
        &token_keypair.pubkey(),
        &mint_authority.pubkey(),
        None,
        0,
    )?;

    let tx: Transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, create_token_instruction],
        Some(&mint_authority.pubkey()),
        &[&mint_authority, &token_keypair],
        recent_blockhash,
    );

    client
        .process_transaction_with_commitment(tx, CommitmentLevel::Finalized)
        .await
        .expect("Unable to create token account");
    Ok(token_keypair)
    // 2. Create token account
}
