use borsh::{BorshDeserialize, BorshSerialize};

// The last number is optional, if user does not want to the add bonus number, then it will be zero.
pub type DraftNumbers = [u8; 7];
pub type Guesses = [DraftNumbers; 4];

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum LotoInstruction {
    Initialize(AccountData),
    PurchaseTickets { guesses: Vec<DraftNumbers> },
    SelectWinners(DraftNumbers),
    CloseAccount,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct AccountData {
    pub guesses: Option<Guesses>,
}