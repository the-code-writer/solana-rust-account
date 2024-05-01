use crate::account_data::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DeleteTransaction<'info> {
  #[account(mut)]
  pub transaction_list: Account<'info, TransactionListAccountData>,
  pub user: Signer<'info>,
}

pub fn delete_transaction(ctx: Context<DeleteTransaction>, id: Pubkey) -> Result<()> {
  let account = &mut ctx.accounts.transaction_list;
  let index = account.get_transaction_index(id)?;
  account.transactions.remove(index);
  account.deleted_indexes.push(index as u8);
  account.count -= 1;
  Ok(())
}
