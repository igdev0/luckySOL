use solana_lottery_program::{
    processor::{find_stake_pool_mint_pda, find_stake_pool_vault_pda},
    state::{LotoInstruction, Winner},
    ID,
};
use solana_program_test::ProgramTest;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program, sysvar,
    transaction::Transaction,
};

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

pub fn purchase_ticket_tx(
    program_id: &Pubkey,
    pool_authority: &Keypair,
    player: &Keypair,
    player_pda_address: Pubkey,
    player_token_pda_address: Pubkey,
    pool_vault_account: Pubkey,
    pool_mint_account: Pubkey,
    recent_blockhash: Hash,
    ticket_data: &LotoInstruction,
) -> Transaction {
    let accounts = vec![
        AccountMeta::new(pool_authority.pubkey(), false),
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

    let instruction = Instruction::new_with_borsh(*program_id, &ticket_data, accounts);

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&player.pubkey()),
        &[&player],
        recent_blockhash,
    )
}

pub fn process_winners_tx(
    pool_authority: &Keypair,
    winners_instruction_data: Vec<Winner>,
    winners_accounts: Vec<AccountMeta>,
    recent_blockhash: Hash,
) -> Transaction {
    let (pool_mint_account, ..) =
        find_stake_pool_mint_pda(&solana_lottery_program::ID, &pool_authority.pubkey());
    let (pool_vault_account, ..) =
        find_stake_pool_vault_pda(&solana_lottery_program::ID, &pool_authority.pubkey());

    let instruction_data = LotoInstruction::SelectWinnersAndAirdrop(winners_instruction_data);
    let mut accounts = vec![
        AccountMeta::new(pool_authority.pubkey(), true),
        AccountMeta::new(pool_vault_account, false),
        AccountMeta::new(pool_mint_account, false),
    ];

    winners_accounts.iter().for_each(|account| {
        accounts.push(account.clone());
    });

    let instruction =
        Instruction::new_with_borsh(solana_lottery_program::ID, &instruction_data, accounts);

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&pool_authority.pubkey()),
        &[&pool_authority],
        recent_blockhash,
    )
}

pub fn process_withdraw_tx(
    player_account: &Keypair,
    player_pda_account: Pubkey,
    amount: u64,
    recent_blockhash: Hash,
) -> Transaction {
    let instruction_data = LotoInstruction::Withdraw(amount);
    let accounts = vec![
        AccountMeta::new(player_account.pubkey(), true),
        AccountMeta::new(player_pda_account, false),
    ];

    let instruction =
        Instruction::new_with_borsh(solana_lottery_program::ID, &instruction_data, accounts);

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&player_account.pubkey()),
        &[&player_account],
        recent_blockhash,
    )
}

pub fn close_account_tx(
    player: &Keypair,
    player_pda_address: Pubkey,
    player_token_pda_address: Pubkey,
    pool_authority: &Pubkey,
    recent_blockhash: Hash,
) -> Transaction {
    let instruction_data = LotoInstruction::ClosePlayerAccount;
    let accounts = vec![
        AccountMeta::new(player.pubkey(), true),
        AccountMeta::new(player_pda_address, false),
        AccountMeta::new(player_token_pda_address, false),
        AccountMeta::new(*pool_authority, false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
    ];

    let instruction =
        Instruction::new_with_borsh(solana_lottery_program::ID, &instruction_data, accounts);

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&player.pubkey()),
        &[&player],
        recent_blockhash,
    )
}
