use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction::create_account,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{errors::*, instruction::*, state::*};

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CounterInstruction::unpack(instruction_data)?;

        match instruction {
            CounterInstruction::CreateCounter { bump } => {
                msg!("Instruction: CreateCounter");
                Self::process_create_counter(accounts, program_id, bump)?;
            }
            CounterInstruction::AddOne => {
                msg!("Instruction: AddOne");
                Self::process_add_one(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_create_counter(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        bump: u8,
    ) -> ProgramResult {
        msg!("process_create_counter fn");
        let account_info_iter = &mut accounts.iter();

        let signer_account_info = next_account_info(account_info_iter)?;

        let counter_account_info = next_account_info(account_info_iter)?;

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        let space = 4;

        let rent_minimum_balance = rent.minimum_balance(space);

        invoke_signed(
            &create_account(
                &signer_account_info.key,
                &counter_account_info.key,
                rent_minimum_balance,
                space as u64,
                program_id,
            ),
            &[signer_account_info.clone(), counter_account_info.clone()],
            &[&[b"counter".as_ref(), &[bump]]],
        )?;

        msg!("antes de  unpack_unchecked");

        let mut counter = Counter::unpack_unchecked(&counter_account_info.try_borrow_data()?)?;

        msg!("pasó unpack_unchecked");

        counter.authority = *signer_account_info.key;

        Counter::pack(
            counter.clone(),
            &mut counter_account_info.try_borrow_mut_data()?,
        )?;

        msg!("The counter authority was set to {}", &counter.authority);

        Ok(())
    }

    pub fn process_add_one(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer_account_info = next_account_info(account_info_iter)?;

        let counter_account_info = next_account_info(account_info_iter)?;

        // Chequeo de findProgramAddress

        let mut counter = Counter::unpack_unchecked(&counter_account_info.try_borrow_data()?)?;

        // Access Control
        if signer_account_info.key != &counter.authority {
            return Err(CounterError::NotAuthorized)?;
        }

        if counter_account_info.owner != program_id {
            return Err(CounterError::InvalidCounterAccount)?;
        }

        counter.number += 1;

        Counter::pack(
            counter.clone(),
            &mut counter_account_info.try_borrow_mut_data()?,
        )?;

        msg!("The counter was increased {} times", &counter.number);

        Ok(())
    }
}
