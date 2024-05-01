use anchor_lang::prelude::*;

#[error_code]
pub enum TransactionError {
    #[msg("The length of a Transaction's content should not be greater than 200")]
    TransactionContentTooLarge,
    #[msg("The length of the Transaction List should not be greater than 8")]
    TransactionListTooLarge,
    #[msg("The Transaction with the given id is not found")]
    TransactionNotFound,
}

#[error_code]
pub enum UserError {
    #[msg("Your PIN is not valid")]
    UserPINError,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    AmountTooSmall,

    #[msg("Withdraw amount cannot be less than deposit")]
    AmountTooBig,
}
