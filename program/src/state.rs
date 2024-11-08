use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum Instruction {
    InitializePool(PoolStorageData),
    Deposit(u64),
    Withdraw(u64),
    PurchaseTicket(TicketAccountData),
    SelectWinnersAndAirdrop(Vec<DraftWinner>),
    ClosePlayerAccount,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TicketAccountData {
    // The merkle root of the ticket stored offchain
    pub merkle_root: [u8; 32],
    pub total_tickets: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct DraftWinner {
    pub amount: u64,
    pub proof: Vec<u8>, // The proof bytes
    pub ticket_indices: Vec<usize>,
    pub tickets: Vec<[u8; 32]>,
    pub address: Pubkey,
    pub token_account: Pubkey,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct PoolStorageData {
    pub ticket_price: u64, // in lamports
    pub draft_count: u64,
    pub initial_amout: u64,
}

pub enum PoolStorageSeed {
    // The stake pool storage seed is used to create the PDA for the stake pool,
    // owned by the spl_token_2022 program and then used as the mint for the receipt mint.
    StakePool,
    ReceiptMint,
    StakeHouse,
    PlayerAccount,
    PlayerTokenAccount,
}

impl PoolStorageSeed {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            PoolStorageSeed::StakePool => "StakePool".as_bytes(),
            PoolStorageSeed::ReceiptMint => "ReceiptMint".as_bytes(),
            PoolStorageSeed::StakeHouse => "StakeHouse".as_bytes(),
            PoolStorageSeed::PlayerAccount => "PlayerAccount".as_bytes(),
            PoolStorageSeed::PlayerTokenAccount => "PlayerTokenAccount".as_bytes(),
        }
    }
}

pub struct HousePoolStorage {}
