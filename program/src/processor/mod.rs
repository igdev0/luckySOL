use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

mod process_close_player_account;
mod process_deposit;
mod process_draft;
mod process_player_withdraw;
mod process_pool_initialization;
mod process_purchase_ticket;

pub use process_pool_initialization::find_player_pda_account;
pub use process_pool_initialization::find_stake_pool_mint_pda;
pub use process_pool_initialization::find_stake_pool_vault_pda;
pub use process_pool_initialization::update_player_account;

pub use process_draft::process_draft;
pub use process_pool_initialization::process_pool_initialization;

pub use process_deposit::process_deposit;

pub use process_purchase_ticket::find_player_token_pda_account;
pub use process_purchase_ticket::process_ticket_purchase;

use crate::state::Instruction;

pub fn processor(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instr = Instruction::try_from_slice(instruction_data)?;
    match instr {
        Instruction::InitializePool(pool_storage_account) => {
            process_pool_initialization(program_id, accounts, &pool_storage_account)
        }
        Instruction::Deposit(amount) => process_deposit(program_id, accounts, amount),
        Instruction::Withdraw(amount) => {
            process_player_withdraw::process_player_withdraw(program_id, accounts, amount)
        }
        Instruction::PurchaseTicket(account_data) => {
            process_ticket_purchase(program_id, accounts, account_data)
        }
        Instruction::ClosePlayerAccount => {
            process_close_player_account::process_close_player_account(program_id, accounts)
        }
        Instruction::SelectWinnersAndAirdrop(draft_winners) => {
            process_draft(program_id, &accounts.to_vec(), draft_winners)
        }
    }
}
