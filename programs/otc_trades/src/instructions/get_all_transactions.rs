use crate::account_data::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetAllTransactions<'info> {
  #[account(mut)]
  pub transaction_list: Account<'info, TransactionListAccountData>,
  pub user: Signer<'info>,
}

pub fn get_all_transactions(ctx: Context<GetAllTransactions>, id: Pubkey) -> Result<()> {
  let account = &mut ctx.accounts.transaction_list;
  let _index = account.get_transaction_index(id)?;
  Ok(())
}
