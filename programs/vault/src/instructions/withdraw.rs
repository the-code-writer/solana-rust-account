use anchor_lang::prelude::*;
use crate::logs::errors::ErrorCode;
use crate::state::bank_account::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct WithdrawAmount<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [BANK_ACCOUNT_SEED, thread_id.as_ref()], bump)]
    pub bank_account: Account<'info, BankAccount>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_handler(ctx: Context<WithdrawAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
    let bank_account = &mut ctx.accounts.bank_account;

    if amount > bank_account.balance {
        return Err(error!(ErrorCode::AmountTooBig));
    };

    bank_account.balance -= amount;
    Ok(())
}
