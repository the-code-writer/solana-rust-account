use anchor_lang::prelude::*;

use accounts::*;
use cryptography::*;
use instructions::*;
use libraries::*;
use setup::*;
use vault::*;
use account_data::*;
use constant::*;
use errors::*;

mod accounts;
mod cryptography;
mod instructions;
mod libraries;
mod setup;
mod vault;
mod account_data;
mod constant;
mod errors;

declare_id!("26PFg6NB6bLYFXJYHdVUxzdYGqa3ceoURsGCDtEo9wtm");

/* 
#[derive(Debug, Deserialize, PartialEq, Clone)]
enum TransactionType {
   #[serde(rename = "deposit")]
   Deposit,
   #[serde(rename = "withdrawal")]
   Withdrawal
}
*/

#[program]
pub mod otc_trades {
    
    use super::*;

    pub fn initialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<UserAccountData> {
        accounts::initialize_user_account(ctx)
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>, content: String) -> Result<()> {
        instructions::execute_transaction(ctx, content)
        /*
        match trans.transaction_type {
            crate::TransactionType::Deposit => {
                wd_result.insert(tid, trans);
            },
            crate::TransactionType::Withdrawal => {
                wd_result.insert(tid, trans);
            }
        }
        */
    }

    pub fn create_transaction(ctx: Context<CreateTransaction>, content: String) -> Result<()> {
        instructions::create_transaction(ctx, content)
    }

    pub fn get_transaction(ctx: Context<GetTransaction>, id: Pubkey) -> Result<()> {
        instructions::get_transaction(ctx, id)
    }

    pub fn get_all_transactions(ctx: Context<GetAllTransactions>, id: Pubkey) -> Result<()> {
        instructions::get_all_transactions(ctx, id)
    }

    pub fn update_transaction(ctx: Context<UpdateTransaction>, transaction: Transaction) -> Result<()> {
        instructions::update_transaction(ctx, transaction)
    }

    pub fn patch_transaction(ctx: Context<PatchTransaction>, transaction: Transaction) -> Result<()> {
        instructions::patch_transaction(ctx, transaction)
    }

    pub fn delete_transaction(ctx: Context<DeleteTransaction>, id: Pubkey) -> Result<()> {
        instructions::delete_transaction(ctx, id)
    }

    pub fn encrypt_text(ctx: Context<EncryptAES256>, text: String) -> Result<String> {
        let encrypted_aes256_text = cryptography::encrypt_aes256::handler(ctx, text);
        encrypted_aes256_text
    }

    pub fn decrypt_text(ctx: Context<DecryptAES256>, text: String) -> Result<String> {
        let decrypted_aes256_text = cryptography::decrypt_aes256::handler(ctx, text);
        decrypted_aes256_text
    }

    /* 

    pub fn initialize_account(
        ctx: Context<Initialize>,
        thread_id: Vec<u8>,
        holder_name: String,
        balance: f64,
    ) -> Result<()> {
        instructions::initialize_account::handler(ctx, thread_id, holder_name, balance)
    }

    pub fn add_interest(ctx: Context<AddInterest>, _thread_id: Vec<u8>) -> Result<()> {
        instructions::add_interest::handler(ctx, _thread_id)
    }

    pub fn withdraw(ctx: Context<WithdrawAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
        instructions::withdraw::handler(ctx, _thread_id, amount)
    }

    pub fn deposit(ctx: Context<DepositAmount>, _thread_id: Vec<u8>, amount: f64) -> Result<()> {
        instructions::deposit::handler(ctx, _thread_id, amount)
    }

    pub fn remove_account(ctx: Context<RemoveAccount>, _thread_id: Vec<u8>) -> Result<()> {
        instructions::remove_account::handler(ctx, _thread_id)
    }

    */

}
