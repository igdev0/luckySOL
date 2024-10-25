use std::ops::Deref;

use solana_lottery_program::{
    processor::{find_stake_pool_mint_pda, find_stake_pool_vault_pda},
    state::{LotoInstruction, PoolStorageSeed},
    ID,
};
use solana_program_test::ProgramTest;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    commitment_config::CommitmentLevel,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program, sysvar,
    transaction::Transaction,
};

type Res<T> = Result<T, ProgramError>;

pub async fn setup() -> (BanksClient, Keypair, solana_sdk::hash::Hash, Keypair) {
    let mut program = ProgramTest::new(
        "solana_lottery_program",
        ID,
        processor!(solana_lottery_program::processor::processor),
    );

    let player = Keypair::new();
    program.add_account(
        player.pubkey(),
        Account::new(100_000_000_000, 0, &system_program::ID),
    );

    // solana_logger::setup_with_default("solana=trace");
    let (banks_client, payer, recent_blockhash) = program.start().await;

    (banks_client, payer, recent_blockhash, player)
}

pub fn initialize_stake_pool_tx(
    program_id: &Pubkey,
    pool_authority: &Keypair,
    recent_blockhash: &Hash,
) -> Transaction {
    let (pool_mint_account, ..) = find_stake_pool_mint_pda(&program_id, &pool_authority.pubkey());
    let (pool_vault_account, ..) = find_stake_pool_vault_pda(&program_id, &pool_authority.pubkey());

    let instruction_data = LotoInstruction::InitializePool(100_000_500);
    let accounts = vec![
        AccountMeta::new(pool_authority.pubkey(), true),
        AccountMeta::new(pool_vault_account, false),
        AccountMeta::new(pool_mint_account, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let instruction =
        Instruction::new_with_borsh(program_id.to_owned(), &instruction_data, accounts);

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&pool_authority.pubkey()),
        &[&pool_authority],
        recent_blockhash.to_owned(),
    )
}
