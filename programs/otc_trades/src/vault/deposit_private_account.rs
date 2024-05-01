use anchor_lang::prelude::*;
use anchor_lang::system_program;
use user_interactions::*;
use crate::errors::ErrorCode;
use crate::state::*;

pub const PRIVATE_ACCOUNT_VAULT_ACCOUNT_SEED: &[u8] = b"private_account_vault_account";
pub const THREAD_AUTHORITY_SEED: &[u8] = b"authority";

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds=[b"vault", signer.key().as_ref()], bump)]
    pub user_vault_account: AccountInfo<'info>,
    #[account(init_if_needed, space = 16 + 8, seeds=[b"counter", signer.key().as_ref()], bump, payer = signer)]
    pub user_interactions_counter: Account<'info, UserInteractions>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct DepositAmount<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(mut, seeds = [PRIVATE_ACCOUNT_VAULT_ACCOUNT_SEED, thread_id.as_ref()], bump)]
    pub private_account_vault_account: Account<'info, PrivateAccountVaultAccount>,

    pub system_program: Program<'info, System>,
}


pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.user_vault_account.to_account_info(),
            },
        ),
        amount,
    )?;

    ctx.accounts.user_interactions_counter.total_deposits += 1;

    Ok(())
}

pub fn handler(ctx: Context<DepositAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
    if amount < 0.0 {
        return Err(error!(ErrorCode::AmountTooSmall));
    };

    let private_account_vault_account = &mut ctx.accounts.private_account_vault_account;
    private_account_vault_account.balance += amount;
    Ok(())
}