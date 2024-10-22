use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum LotoInstruction {
    InitializePool(u64),
    Deposit(u64),
    PurchaseTicket(TicketAccountData),
    SelectWinnersAndAirdrop(),
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
    // The user account which has the authority to validate tickets and airdrop prizes from the stake pool.
    pub stake_pool_authority: Pubkey,
    // The recipe token, used to send back a recipe depending on the amount of tickets bought, 1 token per ticket.
    pub receipt_mint: Pubkey,
    // The user owning the recipe token.
    pub receipt_mint_authority: Pubkey,
    // pub fees_pool_authority: Pubkey,
}

pub enum PoolStorageSeed {
    StakePool,
    StakeHouse,
    // FeesPool,
}

impl PoolStorageSeed {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            PoolStorageSeed::StakePool => "StakePool".as_bytes(),
            PoolStorageSeed::StakeHouse => "StakeHouse".as_bytes(),
            // PoolStorageSeed::FeesPool => "FeesPool".as_bytes(),
        }
    }
}

pub struct HousePoolStorage {}
