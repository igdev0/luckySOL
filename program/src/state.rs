use borsh::{BorshDeserialize, BorshSerialize};

// The last number is optional, if user does not want to the add bonus number, then it will be zero.

pub type DraftNumbers = [u8; 7];
pub type Ticket = [DraftNumbers; 4];

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum LotoInstruction {
    Initialize(TicketAccountData),
    PurchaseTicket { ticket: Ticket },
    SelectWinners(DraftNumbers),
    CloseAccount,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TicketAccountData {
    // The account index, when fully filled the program will create a new account
    pub account_index: usize,
    // The actual tickets containing guesses, maximum 4 guesses per ticket
    pub ticket: Ticket,
    // The number of tickets filled, scluding the empty tickets
}
