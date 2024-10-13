use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum LotoInstruction {
    InitializePool,
    PurchaseTicket(TicketAccountData),
    SelectWinnersAndAirdrop(),
    CloseAccount,
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
    pub referee: Pubkey,
}
