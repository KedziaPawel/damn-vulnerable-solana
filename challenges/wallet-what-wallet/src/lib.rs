#![allow(unexpected_cfgs)]

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod instruction;
pub mod processor;
pub mod state;

use processor::process_instruction as process_instruction_impl;

// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    process_instruction_impl(program_id, accounts, instruction_data)
}
