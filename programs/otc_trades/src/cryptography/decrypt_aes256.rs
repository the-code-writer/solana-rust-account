use anchor_lang::prelude::*;
use crate::accounts::user_account_data_data::*;

use aead::{
    Aead,
    Aes256Gcm
  };

#[derive(Accounts)]
pub struct DecryptAES256<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account_data: Account<'info, UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DecryptAES256>, message: String) -> Result<String> {
    let user_account_data: &Account<'_, UserAccountData> = &ctx.accounts.user_account_data;
    let cipher = Aes256Gcm::new(&user_account_data.key.into());
    let decrypted_message: Vec<u8> = cipher.decrypt(&user_account_data.nonce.into(), message).unwrap();
    Ok(String::from_utf8(decrypted_message).unwrap())
}
