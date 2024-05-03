use anchor_lang::prelude::*;
use crate::logs::errors::ErrorCode;
use crate::state::bank_account::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct DepositAmount<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [BANK_ACCOUNT_SEED, thread_id.as_ref()], bump)]
    pub bank_account: Account<'info, BankAccount>,

    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(ctx: Context<DepositAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
    if amount < 0.0 {
        return Err(error!(ErrorCode::AmountTooSmall));
    };

    let bank_account = &mut ctx.accounts.bank_account;
    bank_account.balance += amount;
    Ok(())
}
