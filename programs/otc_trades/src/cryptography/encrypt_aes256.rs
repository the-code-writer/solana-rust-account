use anchor_lang::prelude::*;
use crate::{
    constant::*,
    states::user_state::UserAccountData,
};

// use aead::{
//   Aead,
//   Aes256Gcm,
//   Key,
//   Nonce,
// };

#[derive(Accounts)]
pub struct EncryptAES256<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account_data: Account<'info, UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EncryptAES256>, message: String) -> Result<String> {
//   let user_account_data: &mut Account<'_, UserAccountData> = &mut ctx.accounts.user_account_data;
//   let encrypted_message: String = Aes256Gcm.encrypt(message, &user_account_data.nonce.into(), &user_account_data.key.into()).unwrap();
//   Ok(encrypted_message)
Ok(message)
}
