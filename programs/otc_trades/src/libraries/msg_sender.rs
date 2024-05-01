use anchor_lang::prelude::*;

use solana_program::{
    msg,
    program_error::ProgramError,
};

pub fn public_key() -> Result<Pubkey, ProgramError> {
    Ok(msg::source()?)
}