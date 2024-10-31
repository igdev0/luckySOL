use crate::{
    error::LotteryError,
    state::{PoolStorageData, PoolStorageSeed, TicketAccountData},
};
use borsh::{to_vec, BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use solana_program::program_pack::Pack;

pub fn process_pool_initialization(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_storage_data: &PoolStorageData,
) -> ProgramResult {
    let mut accounts = accounts.into_iter();

    // The stake pool authority, is the authority which can verify tickets if they are valid and proceeding to airdrop prises.
    let pool_authority_account = next_account_info(&mut accounts)?;

    let pool_vault_account = next_account_info(&mut accounts)?;

    // The PDA vault of the stake pool which will be used to store the tickets.
    let mint_account = next_account_info(&mut accounts)?;

    // The rent account
    let rent_account = next_account_info(&mut accounts)?;

    // The spl_token_2022 account
    let spl_token_2022_account = next_account_info(&mut accounts)?;

    // The system account
    let system_program_account = next_account_info(&mut accounts)?;

    initialize_pool_vault(
        program_id,
        pool_authority_account,
        pool_vault_account,
        system_program_account,
        pool_storage_data,
    )?;

    initialize_pool_mint(
        program_id,
        pool_authority_account,
        mint_account,
        rent_account,
        system_program_account,
        spl_token_2022_account,
        pool_storage_data.initial_amout,
    )?;
    Ok(())
}

fn initialize_pool_vault<'a>(
    program_id: &Pubkey,
    pool_authority_account: &AccountInfo<'a>,
    pool_vault_account: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    pool_storage_data: &PoolStorageData,
) -> ProgramResult {
    let (pool_vault_address, bump) =
        find_stake_pool_vault_pda(program_id, &pool_authority_account.key);

    if pool_vault_account.key != &pool_vault_address {
        return Err(LotteryError::InvalidStakePoolVault.into());
    }

    let rent = Rent::get()?;

    let exempt_balance = rent.minimum_balance(size_of::<PoolStorageData>());

    if pool_authority_account.lamports() < (exempt_balance + pool_storage_data.initial_amout) {
        return Err(LotteryError::InsufficientFunds.into());
    }

    let pool_vault_account_instr = system_instruction::create_account(
        &pool_authority_account.key,
        &pool_vault_address,
        exempt_balance + pool_storage_data.initial_amout,
        size_of::<PoolStorageData>() as u64,
        &program_id,
    );

    invoke_signed(
        &pool_vault_account_instr,
        &[
            pool_authority_account.clone(),
            pool_vault_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            PoolStorageSeed::StakePool.as_bytes(),
            &pool_authority_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    pool_vault_account
        .try_borrow_mut_data()?
        .copy_from_slice(&to_vec(pool_storage_data)?);

    Ok(())
}

fn initialize_pool_mint<'a>(
    program_id: &Pubkey,
    pool_authority_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    rent_account: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    spl_token_2022_account: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    let (pool_mint_address, bump) =
        find_stake_pool_mint_pda(program_id, &pool_authority_account.key);

    if &pool_mint_address != mint_account.key {
        return Err(LotteryError::InvalidStakePoolVault.into());
    }

    let rent = Rent::get()?;

    let exempt_balance = rent.minimum_balance(spl_token_2022::state::Mint::LEN);

    if pool_authority_account.lamports() < exempt_balance {
        return Err(LotteryError::InsufficientFunds.into());
    }

    let mint_account_instr: solana_program::instruction::Instruction =
        system_instruction::create_account(
            &pool_authority_account.key,
            &mint_account.key,
            exempt_balance + amount,
            spl_token_2022::state::Mint::LEN as u64,
            &spl_token_2022::ID,
        );

    invoke_signed(
        &mint_account_instr,
        &[
            pool_authority_account.clone(),
            mint_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            PoolStorageSeed::ReceiptMint.as_bytes(),
            &pool_authority_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    let token_init_instruction = spl_token_2022::instruction::initialize_mint(
        &spl_token_2022::ID,
        &mint_account.key,
        &mint_account.key,
        None,
        0,
    )?;

    invoke_signed(
        &token_init_instruction,
        &[
            pool_authority_account.clone(),
            mint_account.clone(),
            spl_token_2022_account.clone(),
            rent_account.clone(),
        ],
        &[&[
            PoolStorageSeed::ReceiptMint.as_bytes(),
            &pool_authority_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    Ok(())
}

pub fn find_stake_pool_vault_pda(
    program_id: &Pubkey,
    pool_authority_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PoolStorageSeed::StakePool.as_bytes(),
            pool_authority_address.as_ref(),
        ],
        program_id,
    )
}

pub fn update_player_account<'a>(
    player_pda_account: &AccountInfo<'a>,
    data: TicketAccountData,
) -> ProgramResult {
    let mut player_account_data = player_pda_account.try_borrow_mut_data()?;

    let mut ticket_data = TicketAccountData::try_from_slice(&player_account_data)?;

    ticket_data.merkle_root = data.merkle_root;
    ticket_data.total_tickets = data.total_tickets;

    ticket_data.serialize(&mut &mut player_account_data[..])?;

    Ok(())
}

pub fn find_stake_pool_mint_pda(
    program_id: &Pubkey,
    pool_authority_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PoolStorageSeed::ReceiptMint.as_bytes(),
            pool_authority_address.as_ref(),
        ],
        program_id,
    )
}

pub fn find_player_pda_account(
    program_id: &Pubkey,
    player_account: &Pubkey,
) -> (Pubkey, u8, Vec<Vec<u8>>) {
    // Create a vector of owned byte vectors
    let player_account_seeds = vec![
        PoolStorageSeed::PlayerAccount.as_bytes().to_vec(),
        player_account.to_bytes().to_vec(),
    ];

    // Convert references to slices for the `find_program_address` function
    let seeds_refs: Vec<&[u8]> = player_account_seeds
        .iter()
        .map(|seed| seed.as_slice())
        .collect();

    // Find the program address based on the seeds
    let (key, bump) = Pubkey::find_program_address(&seeds_refs, program_id);

    // Return the key, bump, and the owned player account seeds
    (key, bump, player_account_seeds)
}
