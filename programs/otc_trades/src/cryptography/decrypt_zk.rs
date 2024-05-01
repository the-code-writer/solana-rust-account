use anchor_lang::prelude::*;
use crate::accounts::user_account_data::*;

use rand::Rng;
use zk_snark_rs::{Curve, Point, Encryption};

#[derive(Accounts)]
pub struct DecryptZKSnark<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account_data: Account<'info, UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DecryptZKSnark>, message: String) -> Result<String> {
  let user_account_data: &mut Account<'_, UserAccountData> = &mut ctx.accounts.user_account_data;
  let decrypted_message: String = Encryption::decrypt(
    message.as_bytes(),
    &user_account_data.public_key,
    &user_account_data.private_key,
  )
  Ok(decrypted_message)
}
