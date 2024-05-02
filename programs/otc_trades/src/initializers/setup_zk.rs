use anchor_lang::prelude::*;

use crate::{
    constant::*,
    states::*
};

// use rand::Rng;
// use zk_snark_rs::{Curve, Point};

#[derive(Accounts)]
pub struct SetupZKSnark<'info> {
    #[account(init, payer = payer, space = 8 + USER_ACCOUNT_DATA_MAX_SIZE)]
    pub user_account: Account<'info, user_state::UserAccountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetupZKSnark>) -> Result<(String, String)> {
    // let user_account = &mut ctx.accounts.user_account_data;
    // let mut rng = rand::thread_rng();
    // let private_key = rng.gen::<u64>() % Curve::main().order();
    // let public_key = Curve::main().generator() * private_key;
    // user_account.private_key = private_key;
    // user_account.public_key = public_key;
    // Ok((public_key, private_key))
    Ok(("AAA".to_string(),"BBB".to_string()))
}
