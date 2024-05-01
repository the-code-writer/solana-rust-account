use crate::account_data::*;
use crate::constant::*;
use crate::libraries::*;
use crate::errors::*;
use anchor_lang::prelude::*;

use chrono::*;
use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::Utc;

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    #[account(mut)]
    pub transaction_list: Account<'info, TransactionListAccountData>,
    pub user: Signer<'info>,
}

pub fn create_transaction(
    ctx: Context<CreateTransaction>,
    uuid: String,
    title: String,
    description: String,
    unlocks_on: i64,
    expires_on: i64,
    sender: String,
    amount_out: u64,
    token_out: String,
    receiver: String,
    amount_in: u64,
    token_in: String,
    withdrawn: u64,
    claimed: bool,
    unlocked: bool,
    expired: bool,
    cancellable: bool,
    completed: bool,
) -> Result<()> {

    // Validate the title length
    require!(title.len() <= MAX_TRANSACTION_TITLE_LENGTH, TransactionError::TitleTooLong);

    // Validate the description length
    require!(
        description.len() <= MAX_TRANSACTION_CONTENT_LENGTH,
        TransactionError::TransactionContentTooLarge
    );

    // Parse the dates
    let expires_on_date: NaiveDate = NaiveDate::parse_from_str(expires_on, "%Y-%m-%d").unwrap();
    let unlocks_on_date: NaiveDate = NaiveDate::parse_from_str(unlocks_on, "%Y-%m-%d").unwrap();

    // Get the current date
    let current_date: NaiveDate = Utc::now().naive_local().date();
    
    // Validate unlocks_on
    require!(
        unlocks_on_date > current_date,
        TransactionError::InvalidUnlocksOn
    );

    require!(
        chrono::NaiveDateTime::from_timestamp_opt(unlocks_on, 0).is_some(),
        TransactionError::InvalidUnlocksOn
    );

    // Validate expires_on
    require!(
        expires_on_date > current_date,
        TransactionError::InvalidExpiresOn
    );

    require!(
        chrono::NaiveDateTime::from_timestamp_opt(expires_on, 0).is_some(),
        TransactionError::InvalidExpiresOn
    );

    // Validate unlocks_on and expires_on :: expires_on should be greater than unlocks on
    require!(
        expires_on_date > unlocks_on_date,
        TransactionError::ExpiresOnDateMustBeGreaterThanUnlocksOn
    );

    // Validate sender
    require!(
        libraries::validate_addresses::is_valid_encrypted_address(&sender),
        TransactionError::InvalidSender
    );

    // Validate amount_out
    require!(
        amount_out > 0 && ctx.accounts.sender_balance >= amount_out,
        TransactionError::InvalidAmountOut
    );

    // Validate token_out
    require!(
        libraries::validate_addresses::is_valid_wallet_address(&token_out) && ctx.accounts.sender_balance >= amount_out,
        TransactionError::InvalidTokenOut
    );

    // Validate receiver
    require!(
        libraries::validate_addresses::is_valid_encrypted_address(&receiver),
        TransactionError::InvalidReceiver
    );

    // Validate amount_in
    require!(amount_in > 0, TransactionError::InvalidAmountIn);

    // Validate token_in
    require!(
        libraries::validate_addresses::is_valid_wallet_address(&token_in),
        TransactionError::InvalidTokenIn
    );

    let account: &mut Account<'_, TransactionListAccountData> = &mut ctx.accounts.transaction_list;

    let nonce: u8 = account.transactions_count;

    require!(
        (nonce as usize) < MAX_TRANSACTION_LIST_LENGTH,
        TransactionError::TransactionListTooLarge
    );

    let index: u8 = match account.deleted_indexes.pop() {
        Some(value) => value,
        None => nonce,
    };

    let (id, _) = Pubkey::find_program_address(&[b"transaction", &[index]], ctx.program_id);

    account.transactions_count += 1;

    account.transactions.push(Transaction {
        id,
        uuid,
        nonce,
        title,
        description,
        unlocks_on,
        expires_on,
        sender,
        amount_out,
        token_out,
        receiver,
        amount_in,
        token_in,
        withdrawn: 0,
        unlocked: false,
        expired: false,
        cancellable: false,
        completed: false,
        claims: [],
    });

    Ok(())
    
}
