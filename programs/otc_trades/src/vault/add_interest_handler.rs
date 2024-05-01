use anchor_lang::prelude::*;
use clockwork_sdk::state::Thread;
use crate::state::*;

pub const PRIVATE_VAULT_ACCOUNT_SEED: &[u8] = b"private_vault_account";
pub const THREAD_AUTHORITY_SEED: &[u8] = b"authority";

// Calculating interest per minute instead of anually for faster results
const MINUTE_INTEREST: f64 = 0.05; // 5% interest return

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct AddInterest<'info> {
    #[account(mut, seeds = [PRIVATE_VAULT_ACCOUNT_SEED, thread_id.as_ref()], bump)]
    pub private_vault_account: Account<'info, PrivateVaultAccount>,

    #[account(signer, constraint = thread.authority.eq(&thread_authority.key()))]
    pub thread: Account<'info, Thread>,

    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}

pub fn handler(ctx: Context<AddInterest>, _thread_id: Vec<u8>) -> Result<()> {
    let now = Clock::get().unwrap().unix_timestamp;

    let private_vault_account = &mut ctx.accounts.private_vault_account;
    private_vault_account.updated_at = now;

    let elapsed_time = (now - private_vault_account.created_at) as f64;
    let minutes = elapsed_time / 60.0;
    let accumulated_value = private_vault_account.balance * (1.0 + (MINUTE_INTEREST)).powf(minutes);

    private_vault_account.balance = accumulated_value;

    msg!(
        "New Balance: {}, Minutes Elasped when Called: {}",
        accumulated_value,
        minutes,
    );
    Ok(())
}