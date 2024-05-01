use crate::account_data::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetTransaction<'info> {
  #[account(mut)]
  pub transaction_list: Account<'info, TransactionListAccountData>,
  pub user: Signer<'info>,
}

pub fn get_transaction(ctx: Context<GetTransaction>, id: Pubkey) -> Result<()> {
  let account = &mut ctx.accounts.transaction_list;
  let _index = account.get_transaction_index(id)?;
  Ok(())
}
