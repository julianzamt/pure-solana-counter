use std::option::Iter;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction::create_account,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::*, state::*};

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::unpack(instruction_data)?;

        match instruction {
            CounterInstruction::AddOne {} => {
                msg!("Instruction: AddOne");
                Self::process_add_one(accounts, program_id);
            }
        }

        Ok(())
    }

    pub fn process_add_one(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;

        let counter = next_account_info(account_info_iter)?;

        // Init if needed
        if counter.lamports() == 0 && *counter.owner == solana_program::system_program::id() {
            let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

            let space = 4;

            let rent_minimum_balance = rent.minimum_balance(space);

            invoke_signed(
                &create_account(
                    &signer.key,
                    &counter.key,
                    rent_minimum_balance,
                    space as u64,
                    program_id,
                ),
                &[signer.clone(), counter.clone()],
                &[&[b"counter".as_ref(), &[bump]]],
            )?;
        }
        Ok(())
    }
}
