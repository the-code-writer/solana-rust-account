use anchor_lang::prelude::*;

use crate::{
    cryptography::*,
    cryptography::encrypt_zk::*,
    constant::*,
    initializers::setup_aes256,
    initializers::setup_aes256::SetupAES256,
    initializers::setup_zk,
    initializers::setup_zk::SetupZKSnark,
    initializers::setup_firebase,
    initializers::setup_firebase::SetupFirebase,
    states::*
};

use std::time::SystemTime;

#[derive(Accounts)]
pub struct InitializeUserAccount<'info> {
  #[account(
    init, 
    seeds = [authority.key().as_ref(), USER_ACCOUNT_DATA_SEED],
    bump,
    payer = authority, 
    space = 8 + USER_ACCOUNT_DATA_MAX_SIZE
  )]
  pub user_account_data: Account<'info, user_state::UserAccountData>,
  #[account(mut)]
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub aes256: SetupAES256<'info>,
  pub zk_snark: SetupZKSnark<'info>,
  pub firebase: SetupFirebase<'info>, 
  pub zk_snark_encrypt: EncryptZKSnark<'info>,
}

pub fn initialize_user_account(
  ctx: Context<InitializeUserAccount>,
  data: String
) -> Result<user_state::UserAccountData> {
  let user_account = &mut ctx.accounts.user_account_data;
  let authority = &mut ctx.accounts.authority;
  user_account.authority = authority.key() ;
  setup_aes256::handler(ctx.aes256);
  setup_zk::handler(ctx.zk_snark);
  setup_firebase::handler(ctx.firebase);
  let _created_at= SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs() as u64;
  let _wallet_address = authority.key(); //Pubkey::from(msg::payer()).to_string();
  user_account.wallet_address = _wallet_address;
  user_account.wallet_hash = encrypt_zk::handler(ctx.zk_snark_encrypt, _wallet_address);
  user_account.created_at = _created_at;
  user_account.updated_at = _created_at;
  user_account.private_balance = 0;
  user_account.token_balances = Vec::with_capacity(MAX_TOKEN_HOLDING_LIST_LENGTH);
  user_account.transactions = Vec::with_capacity(MAX_TRANSACTION_LIST_LENGTH);
  user_account.transaction_deleted_indexes = Vec::with_capacity(MAX_TRANSACTION_LIST_LENGTH);
  user_account.transaction_count = 0;
  user_account.is_active = true;
  user_account.data = data;
  Ok(user_account)
}

