use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const BANK_ACCOUNT_SEED: &[u8] = b"bank_account";

#[constant]
pub const THREAD_AUTHORITY_SEED: &[u8] = b"authority";

// Calculating interest per minute instead of anually for faster results
#[constant]
pub const MINUTE_INTEREST: f64 = 0.05; // 5% interest return

// Calculating interest per minute instead of anually for faster results
#[constant]
pub const CRON_SCHEDULE: &str = "*/10 * * * * * *"; // 10s https://crontab.guru/

#[constant]
pub const AUTOMATION_FEE: u64 = 5000000; // https://docs.clockwork.xyz/developers/threads/fees



