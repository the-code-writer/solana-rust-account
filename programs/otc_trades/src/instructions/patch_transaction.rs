use crate::account_data::*;
use crate::constant::*;
use crate::errors::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PatchTransaction<'info> {
  #[account(mut)]
  pub transaction_list: Account<'info, TransactionListAccountData>,
  pub user: Signer<'info>,
}

pub fn patch_transaction(ctx: Context<PatchTransaction>, transaction: Transaction) -> Result<()> {
  require!(
    transaction.content.len() <= MAX_TRANSACTION_CONTENT_LENGTH,
    TransactionError::TransactionContentTooLarge
  );

  let account = &mut ctx.accounts.transaction_list;
  let index = account.get_transaction_index(transaction.id)?;

  *(account.transactions.get_mut(index).expect("")) = transaction;
  Ok(())
}
