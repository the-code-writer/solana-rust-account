use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::state::*;

pub const PRIVATE_VAULT_ACCOUNT_SEED: &[u8] = b"private_vault_account";
pub const THREAD_AUTHORITY_SEED: &[u8] = b"authority";

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct WithdrawAmount<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [PRIVATE_VAULT_ACCOUNT_SEED, thread_id.as_ref()], bump)]
    pub private_vault_account: Account<'info, PrivateVaultAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
    let private_vault_account = &mut ctx.accounts.private_vault_account;

    if amount > private_vault_account.balance {
        return Err(error!(ErrorCode::AmountTooBig));
    };

    private_vault_account.balance -= amount;
    Ok(())
}