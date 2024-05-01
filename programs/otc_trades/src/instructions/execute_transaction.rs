use crate::account_data::*;
use crate::constant::*;
use crate::errors::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
  #[account(mut)]
  pub transaction_list: Account<'info, TransactionListAccountData>,
  pub user: Signer<'info>,
}

pub fn execute_transaction(ctx: Context<ExecuteTransaction>, content: String) -> Result<()> {
  require!(
    content.len() <= MAX_TRANSACTION_CONTENT_LENGTH,
    TransactionError::TransactionContentTooLarge
  );

  let account = &mut ctx.accounts.transaction_list;
  require!(
    (account.count as usize) < MAX_TRANSACTION_LIST_LENGTH,
    TransactionError::TransactionListTooLarge
  );

  let index = match account.deleted_indexes.pop() {
    Some(value) => value,
    None => account.count,
  };
  let (id, _) = Pubkey::find_program_address(&[b"transaction", &[index]], ctx.program_id);
  account.count += 1;
  account.transactions.push(Transaction {
    id,
    content,
    completed: false,
  });
  Ok(())
}
