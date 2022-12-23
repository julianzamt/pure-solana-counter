use crate::errors::CounterError::InvalidInstruction;
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum CounterInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` payer
    /// 1. `[]` The counter pubkey
    /// 2. `[]` The rent sysvar
    /// 3. `[]` The system program
    CreateCounter { bump: u8 },
    /// Accounts expected:
    ///
    /// 0. `[signer]`
    /// 1. `[writable]` The account that stores the accumulator
    AddOne,
}

#[derive(BorshDeserialize, Debug)]
struct Payload {
    variant: u8,
    bump: u8,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("unpack");

        let payload = Payload::try_from_slice(input).unwrap();
        Ok(match payload.variant {
            0 => Self::CreateCounter { bump: payload.bump },
            1 => Self::AddOne,

            _ => return Err(InvalidInstruction)?,
        })
    }
}
