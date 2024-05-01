use anchor_lang::prelude::*;
use crate::accounts::user_account_data::*;

use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

// Define a distribution for special characters
struct StandardSpecial;

impl DistString for StandardSpecial {
    fn new() -> Self {
        Self
    }
    fn sample<R>(&self, rng: &mut R) -> String
    where
        R: Rng,
    {
        let special_chars = "!@#$%^&*()_+-={}|[]:;<>?,./~";
        let idx = rng.gen_range(0..special_chars.len());
        special_chars.chars().nth(idx).unwrap().to_string()
    }
}

// Define a distribution for digits
struct Digits;

impl DistString for Digits {
    fn new() -> Self {
        Self
    }
    fn sample<R>(&self, rng: &mut R) -> String
    where
        R: Rng,
    {
        let digits = "0123456789";
        let idx = rng.gen_range(0..digits.len());
        digits.chars().nth(idx).unwrap().to_string()
    }
}

#[derive(Accounts)]
pub struct SetupCredentials<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account_data: Account<'info, UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetupCredentials>) -> Result<FirebaseCredentials> {
    let user_account_data = &mut ctx.accounts.user_account_data;
    let mut rng = rand::thread_rng();
    let username = format!(
        "fbusr_{}",
        Alphanumeric.new().sample_string(&mut rng, 8).to_lowercase()
    );
    let password = format!(
        "@{}{}{}{}{}{}",
        rng.sample(Alphanumeric),
        rng.sample(Alphanumeric).to_uppercase(),
        rng.sample(StandardSpecial),
        rng.sample(StandardSpecial),
        rng.sample(StandardSpecial),
        rng.sample(Digits),
        rng.sample(Digits),
        rng.sample(Digits)
    );
    let credentials = FirebaseCredentials { username, password };
    user_account_data.credentials = credentials;
    Ok(credentials)
}

pub fn get_firebase_credentials(ctx: Context<SetupAES256>) -> Result<(FirebaseCredentials)> {
    let user_account_data = &mut ctx.accounts.user_account_data;
    Ok(&user_account_data.credentials)
}
