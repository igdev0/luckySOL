use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

mod process_close_player_account;
mod process_deposit;
mod process_player_withdraw;
mod process_pool_initialization;
mod process_purchase_ticket;
mod process_winners;

pub use process_pool_initialization::find_player_pda_account;
pub use process_pool_initialization::find_stake_pool_mint_pda;
pub use process_pool_initialization::find_stake_pool_vault_pda;
pub use process_pool_initialization::update_player_account;

pub use process_pool_initialization::process_pool_initialization;
pub use process_winners::process_winners;

pub use process_deposit::process_deposit;

pub use process_purchase_ticket::find_player_token_pda_account;
pub use process_purchase_ticket::process_ticket_purchase;
pub use process_purchase_ticket::TICKET_PRICE;

use crate::state::LotoInstruction;

pub fn processor(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instr = LotoInstruction::try_from_slice(instruction_data)?;
    match instr {
        LotoInstruction::InitializePool(amount) => {
            process_pool_initialization(program_id, accounts, amount)
        }
        LotoInstruction::Deposit(amount) => process_deposit(program_id, accounts, amount),
        LotoInstruction::Withdraw(amount) => {
            process_player_withdraw::process_player_withdraw(program_id, accounts, amount)
        }
        LotoInstruction::PurchaseTicket(account_data) => {
            process_ticket_purchase(program_id, accounts, account_data)
        }
        LotoInstruction::ClosePlayerAccount => {
            process_close_player_account::process_close_player_account(program_id, accounts)
        }
        LotoInstruction::SelectWinnersAndAirdrop(winners) => {
            process_winners(program_id, &accounts.to_vec(), winners)
        }
    }
}
