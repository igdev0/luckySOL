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
pub struct PoolAccount {
    // The user account which has the authority to validate tickets and airdrop prizes from the stake pool.
    pub stake_pool_authority: Pubkey,
    // The recipe token, used to send back a recipe depending on the amount of tickets bought, 1 token per ticket.
    pub recipe_token: Pubkey,
    // The user owning the recipe token.
    pub recipe_token_owner: Pubkey,
    // The user account wich has the authority to move funds from StakeAccount
    // for now, this is a simple account, in the near future it will be managed by comunity votes
    pub stake_house_authority: Pubkey,
    // // The user which has the authority to move funds from FeesAccount
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
