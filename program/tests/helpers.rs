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
