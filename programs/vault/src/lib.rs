pub mod constants;
pub mod logs;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use logs::*;
pub use state::*;

declare_id!("B9PKQpVHnyHzyfzN759fenv3ArX5csaaZLV448gSe25k");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize_handler(ctx)
    }

    pub fn initialize_account(
        ctx: Context<InitializeAccount>,
        thread_id: Vec<u8>,
        holder_name: String,
        balance: f64,
    ) -> Result<()> {
        instructions::initialize_account::initialize_account_handler(ctx, thread_id, holder_name, balance)
    }

    pub fn add_interest(ctx: Context<AddInterest>, _thread_id: Vec<u8>) -> Result<()> {
        instructions::add_interest::add_interest_handler(ctx, _thread_id)
    }

    pub fn withdraw(ctx: Context<WithdrawAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
        instructions::withdraw::withdraw_handler(ctx, _thread_id, amount)
    }

    pub fn deposit(ctx: Context<DepositAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
        instructions::deposit::deposit_handler(ctx, _thread_id, amount)
    }

    pub fn remove_account(ctx: Context<RemoveAccount>, _thread_id: Vec<u8>) -> Result<()> {
        instructions::remove_account::remove_account_handler(ctx, _thread_id)
    }
    
}
