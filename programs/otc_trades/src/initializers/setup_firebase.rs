use anchor_lang::prelude::*;

use crate::{
    constant::*,
    states::user_state::UserAccountData,
    states::user_state::FirebaseCredentials
};

// use rand::distributions::{Alphanumeric, DistString};
// use rand::Rng;

// Define a distribution for special characters
/* 
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

*/

#[derive(Accounts)]
pub struct SetupFirebase<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account_data: Account<'info, UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetupFirebase>) -> Result<FirebaseCredentials> {
    let user_account_data = &mut ctx.accounts.user_account_data;
    /* 
    let mut rng = rand::thread_rng();
    let username = format!(
        "fb_{}",
        Alphanumeric.new().sample_string(&mut rng, 8).to_lowercase()
    );
    let password = format!(
        "@{}{}{}{}{}{}{}{}",
        rng.sample(Alphanumeric),
        rng.sample(StandardSpecial),
        rng.sample(Digits),
        rng.sample(StandardSpecial),
        rng.sample(Digits),
        rng.sample(Alphanumeric).to_uppercase(),
        rng.sample(Digits),
        rng.sample(StandardSpecial)
    );
    let credentials = FirebaseCredentials { username, password };
    user_account_data.credentials = credentials;
    Ok(credentials)
    */
    Ok(FirebaseCredentials{username:"USERNAME".to_string(), password: "PASSWORD".to_string()})
}

pub fn get_firebase_credentials(ctx: Context<SetupFirebase>) -> Result<FirebaseCredentials> {
    // let user_account_data = &mut ctx.accounts.user_account_data;
    // Ok(&user_account_data.credentials)
    Ok(FirebaseCredentials{username:"USERNAME".to_string(), password: "PASSWORD".to_string()})
}
