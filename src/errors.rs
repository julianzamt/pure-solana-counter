use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum CounterError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Counter Account")]
    InvalidCounterAccount,

    #[error("Not Authorized")]
    NotAuthorized,
}

impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
