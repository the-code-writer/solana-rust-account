use anchor_lang::prelude::*;
use crate::cryptography::*;
use crate::constant::*;
use crate::errors::*;
use std::time::SystemTime;
use pubKey::Pubkey;

#[account]
pub struct TransactionListAccountData {
  pub transactions_count: u8,                // 1
  pub deleted_indexes: Vec<u8>, // 4 + max * 1
  pub transactions: Vec<Transaction>,         // 4 + max * (32 + 4 + 200 +1)
}

impl TransactionListAccountData {
  const COUNT_SIZE: usize = 1;
  const DELETED_INDEXES: usize   = 4 + MAX_TRANSACTION_LIST_LENGTH * 1;
  const TRANSACTIONS_SIZE: usize = 4 + MAX_TRANSACTION_LIST_LENGTH * (32 + 4 + 200 + 1);
  pub const MAX_SIZE: usize = TransactionListAccountData::COUNT_SIZE
    + TransactionListAccountData::DELETED_INDEXES
    + TransactionListAccountData::TRANSACTIONS_SIZE;
  pub fn get_transaction_index(&self, id: Pubkey) -> Result<usize> {
    for (index, transaction) in self.transactions.iter().enumerate() {
      if transaction.id == id {
        return Ok(index);
      }
    }
    err!(TransactionError::TransactionNotFound)
  }
}

#[account]
pub struct Claim {
  pub claimed_on: i64,
  pub claimed_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Transaction {
  pub id: Pubkey,             // 32
  pub uuid: String,           // 4 + 44
  pub nonce: u64,             // 4
  pub title: String,          // 4
  pub description: String,    // 4 + 200
  pub unlocks_on: i64,        // 4
  pub expires_on: i64,        // 4
  pub claims: Vec<Claim>,     // 4
  pub sender: String,         // 4 + 44
  pub amount_out: u64,        // 4
  pub token_out: String,      // 4 + 44
  pub receiver: String,       // 4 + 44
  pub amount_in: u64,         // 4
  pub token_in: String,       // 4 + 44
  pub withdrawn: u64,         // 4
  pub unlocked: bool,         // 1
  pub expired: bool,          // 1
  pub cancellable: bool,      // 1
  pub completed: bool,        // 1
}
