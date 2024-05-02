use anchor_lang::prelude::*;

pub mod accounts_data;
pub mod cryptography;
pub mod initializers;
pub mod constant;
pub mod states;

use crate::{
    accounts_data::user_account::InitializeUserAccount,
    accounts_data::user_account::*,
    states::user_state::UserAccountData,
};

declare_id!("32r6fQPboB25e9E9pRz6xpZmzfuEk5tzoqdxZFkTLiRD");

#[program]
mod oct_trades {
    use super::*;
    pub fn initialize_user_account(
        ctx: Context<InitializeUserAccount>, 
        data: String
    ) -> Result<UserAccountData> {
        initialize_user_account(ctx, data)
    }
}
