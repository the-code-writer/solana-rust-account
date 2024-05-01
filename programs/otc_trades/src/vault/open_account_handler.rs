use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::InstructionData;
use clockwork_sdk::state::Thread;
use crate::state::*;

pub const PRIVATE_VAULT_ACCOUNT_SEED: &[u8] = b"private_vault_account";
pub const THREAD_AUTHORITY_SEED: &[u8] = b"authority";

// Calculating interest per minute instead of anually for faster results
const CRON_SCHEDULE: &str = "*/10 * * * * * *"; // 10s https://crontab.guru/
const AUTOMATION_FEE: u64 = 5000000; // https://docs.clockwork.xyz/developers/threads/fees

#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(
        init,
        payer = holder,
        seeds = [PRIVATE_VAULT_ACCOUNT_SEED, thread_id.as_ref()],
        bump,
        space = 8 + std::mem::size_of::<PrivateVaultAccount>(),
    )]
    pub private_vault_account: Account<'info, PrivateVaultAccount>,

    #[account(mut, address = Thread::pubkey(thread_authority.key(), thread_id))]
    pub thread: SystemAccount<'info>,

    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,

    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    thread_id: Vec<u8>,
    holder_name: String,
    balance: f64,
) -> Result<()> {
    let system_program = &ctx.accounts.system_program;
    let clockwork_program = &ctx.accounts.clockwork_program;

    let holder = &ctx.accounts.holder;
    let private_vault_account = &mut ctx.accounts.private_vault_account;

    let thread = &ctx.accounts.thread;
    let thread_authority = &ctx.accounts.thread_authority;

    private_vault_account.thread_id = thread_id.clone();
    private_vault_account.holder = *holder.key;
    private_vault_account.balance = balance;
    private_vault_account.holder_name = holder_name;
    private_vault_account.created_at = Clock::get().unwrap().unix_timestamp;

    // Clockwork Target Instruction
    let target_ix = Instruction {
        program_id: crate::ID,
        accounts: crate::accounts::AddInterest {
            private_vault_account: private_vault_account.key(),
            thread: thread.key(),
            thread_authority: thread_authority.key(),
        }.to_account_metas(Some(true)),
        data: crate::instruction::AddInterest {
            _thread_id: thread_id.clone(),
        }.data(),
    };

    // Clockwork Trigger
    let trigger = clockwork_sdk::state::Trigger::Cron {
        schedule: CRON_SCHEDULE.to_string(),
        skippable: true,
    };

    // Clockwork thread CPI
    let bump = *ctx.bumps.get("thread_authority").unwrap();
    clockwork_sdk::cpi::thread_create(
        CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadCreate {
                payer: holder.to_account_info(),
                system_program: system_program.to_account_info(),
                thread: thread.to_account_info(),
                authority: thread_authority.to_account_info(),
            },
            &[&[THREAD_AUTHORITY_SEED, &[bump]]],
        ),
        AUTOMATION_FEE,
        thread_id,
        vec![target_ix.into()],
        trigger,
    )?;

    Ok(())
}