use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum LotoInstruction {
    InitializePool(u64),
    Deposit(u64),
    PurchaseTicket(TicketAccountData),
    SelectWinnersAndAirdrop(Vec<Pubkey>),
    ClosePlayerAccount,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TicketAccountData {
    // The merkle root of the ticket stored offchain
    pub merkle_root: [u8; 32],
    // The address of the account purchasing the ticket
    pub address: Pubkey,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct PoolStorageAccount {
    // The recipe token, used to send back a recipe depending on the amount of tickets bought, 1 token per ticket.
    pub receipt_mint: Pubkey,
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
