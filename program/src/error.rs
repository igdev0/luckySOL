use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
enum LotteryError {
    #[error("Incorrect owner set")]
    IncorrectOwner,
}

impl From<LotteryError> for ProgramError {
    fn from(value: LotteryError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
