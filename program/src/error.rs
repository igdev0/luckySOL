use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LotteryError {
    #[error("Incorrect owner set")]
    IncorrectOwner,
    #[error("Not implemented yet.")]
    NotImplemented,
    #[error("Invalid InvalidStakePoolVault")]
    InvalidStakePoolVault,
    #[error("InsufficientFunds")]
    InsufficientFunds,
    #[error("The stake pool is not initialized")]
    StakePoolNotInitialized,
    #[error("Invalid player PDA account")]
    InvalidPlayerPdaAccount,
    #[error("The authority must signn the transaction")]
    AuthorityMustSign,
    #[error("Invalid signer ")]
    InvalidSigner,
    #[error("Invalid account")]
    InvalidAccount,
    #[error("Invalid ticket")]
    InvalidTicket,
    #[error("Invalid owner")]
    InvalidOwner,
    #[error("Invalid program account")]
    InvalidProramAccount,
}

impl From<LotteryError> for ProgramError {
    fn from(value: LotteryError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
