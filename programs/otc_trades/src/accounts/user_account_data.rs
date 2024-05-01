use anchor_lang::prelude::*;
use std::time::SystemTime;
use pubKey::Pubkey;
use crate::cryptography::*;
use crate::libraries::*;
use crate::setup::*;
use crate::constant::*;
use crate::errors::*;

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
    pub private_key: String,
    pub public_key: String,
    pub wallet_hash: String,
    pub wallet_address: Pubkey,
    pub credentials: FirebaseCredentials
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub private_balance: u32,
    pub token_balances: Vec<TokenHolding>,
    pub transactions: Vec<Transaction>,
}

#[derive(Accounts)]
pub struct InitializeUserAccount<'info> {
  #[account(
    init, 
    seed = [USER_ACCOUNT_DATA_SEED, authority.key().as_ref()],
    bump,
    payer = authority, 
    space = 8 + USER_ACCOUNT_DATA_MAX_SIZE
  )]
  pub user_account_data: Account<'info, UserAccountData>,
  #[account(mut)]
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn initialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<UserAccountData> {
  let user_account = &mut ctx.accounts.user_account_data;
  let authority = &mut ctx.accounts.authority;
  setup_aes256::handler();
  setup_zk::handler();
  setup_credentials::handler();
  let _created_at= SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs() as u64;
  let _wallet_address = Pubkey::new(msg_sender::address()).to_string();
  user_account.wallet_address = _wallet_address;
  user_account.wallet_hash = encrypt_zk::handler(_wallet_address);
  user_account.created_at = _created_at;
  user_account.updated_at = _created_at;
  user_account.private_balance = 0;
  user_account.token_balances = Vec<TokenHolding>::with_capacity(MAX_TOKEN_HOLDING_LIST_LENGTH);
  user_account.transactions = Vec<Transaction>::with_capacity(MAX_TRANSACTION_LIST_LENGTH);
  user_account.transaction_deleted_indexes = Vec::with_capacity(MAX_TRANSACTION_LIST_LENGTH);
  user_account.transaction_count = 0;
  user_account.is_active = true;
  Ok(user_account)
}

