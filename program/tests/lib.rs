mod helpers;

use solana_lottery_program::state::{LotoInstruction, PoolStorageSeed};
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

    let (pool_mint_account, _bump) = Pubkey::find_program_address(
        &[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority.pubkey().to_bytes(),
        ],
        &program_id,
    );

    let instruction_data = LotoInstruction::InitializePool(100_000_500);
    let accounts = vec![
        AccountMeta::new(pool_authority.pubkey(), true),
        AccountMeta::new(pool_mint_account, false),
        AccountMeta::new_readonly(sysvar::rent::ID, false),
        AccountMeta::new_readonly(spl_token_2022::ID, false),
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
