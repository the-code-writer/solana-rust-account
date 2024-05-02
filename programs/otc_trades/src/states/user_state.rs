use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Transaction {
  pub id: Pubkey,             // 32
  pub uuid: String,           // 4 + 44
  pub nonce: u64,             // 4
  pub title: String,          // 4
  pub description: String,    // 4 + 200
  pub unlocks_on: u64,        // 4
  pub expires_on: u64,        // 4
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

#[account]
#[derive(Default)]
pub struct Claim {
  timestamp: String,
  token_address: String,
  amount: String,
}

#[account]
#[derive(Default)]
pub struct FirebaseCredentials {
  username: String,
  password: String,
}

#[account]
#[derive(Default)]
pub struct TokenHolding {
  address: String,
  balance: u64,
}

#[account]
#[derive(Default)]
pub struct UserAccountData {
    pub key: [u8; 32],
    pub nonce: [u8; 12],
    pub authority: Pubkey,
    pub private_key: String,
    pub public_key: String,
    pub wallet_hash: String,
    pub wallet_address: Pubkey,
    pub credentials: FirebaseCredentials,
    pub is_active: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub private_balance: u32,
    pub token_balances: Vec<TokenHolding>,
    pub transactions: Vec<Transaction>,
    pub transaction_deleted_indexes: Vec<u8>,
    pub transaction_count: u8,
    pub data: String,
}
