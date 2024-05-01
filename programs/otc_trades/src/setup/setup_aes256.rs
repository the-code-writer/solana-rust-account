use crate::account_data::*;
use anchor_lang::prelude::*;

use aead::{
  Aead,
  Aes256Gcm,
  Key,
  Nonce,
};

#[derive(Accounts)]
pub struct SetupAES256<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account_data: Account<'info, UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub struct AES256UserKey {
  key: String,
  nonce: String,
}

pub fn handler(ctx: Context<SetupAES256>) -> Result<AES256UserKey> {
    let user_account_data = &mut ctx.accounts.user_account_data;
    // let mut rng = rand::thread_rng();
    // let key: [u8; 32] = rng.gen();
    // let nonce: [u8; 12] = rng.gen();
    // let cipher = Aes256Gcm::new(&key.into());
    // let encrypted_message = cipher.encrypt(&nonce.into(), message.as_bytes()).unwrap();
    let key = Key::new([0u8; 32]); // 32-byte key for Aes256Gcm
    let nonce = Nonce::new([0u8; 12]); // 12-byte nonce
    user_account_data.key = key;
    user_account_data.nonce = nonce;
    let user_key:AES256UserKey = AES256UserKey{
      key, nonce
    }
    Ok(user_key)
  }

  pub fn get_keys(ctx: Context<SetupAES256>) -> Result<AES256UserKey> {
    let user_account_data = &mut ctx.accounts.user_account_data;
    let user_key:AES256UserKey = AES256UserKey{
      key: &user_account_data.key.into(), nonce: &user_account_data.nonce.into()
    }
    Ok(user_key)
  }
    