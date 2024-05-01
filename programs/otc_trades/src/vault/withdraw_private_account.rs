use anchor_lang::prelude::*;
use anchor_lang::system_program;
use user_interactions::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds=[b"vault", signer.key().as_ref()], bump)]
    pub user_vault_account: AccountInfo<'info>,
    #[account(mut, seeds=[b"counter", signer.key().as_ref()], bump)]
    pub user_interactions_counter: Account<'info, UserInteractions>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let bump = *ctx.bumps.get("user_vault_account").unwrap();

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user_vault_account.key(),
        &ctx.accounts.signer.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.user_vault_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
        ],
        &[&[b"vault", ctx.accounts.signer.key().as_ref(), &[bump]]],
    )?;

    ctx.accounts.user_interactions_counter.total_withdrawals += 1;

    Ok(())
}