use lottery_program::{entrypoint, error, processor};
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

#[tokio::test]
async fn initialize_player_account() {
    let program_id = Keypair::new();
    let user_account = Keypair::new();
    let program_test =
        solana_program_test::ProgramTest::new("lottery_program", program_id.pubkey(), None);
}
