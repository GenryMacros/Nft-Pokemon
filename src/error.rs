use thiserror::Error

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum NftError {

    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("No tokens left")]
    NoTokens,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid trade")]
    InvalidTrade,
}

impl From<NftError> for ProgramError {
    fn from(e: NftError) -> Self {
        ProgramError::Custom(e as u32);
    }
}