use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use std::mem::size_of;

// This is your program's public key and it will update
// automatically when you build the project.

declare_id!("3BFJz58cjCRSgbB81bTgvVNGLL4hD27wT1pCFpxBnCoz");

#[program]
mod rocket_revenue {
    use super::*;
    pub fn create_lease(ctx: Context<CreateLease>, amount: u64) -> ProgramResult {
        let lease: &mut Account<LeaseAccount> = &mut ctx.accounts.lease;
        let signer: &Signer = &ctx.accounts.signer;
        let clock = Clock::get().unwrap();

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                    authority: signer.to_account_info(),
                },
            ),
            amount,
        )?;

        lease.signer = *signer.key;
        lease.bump = *ctx.bumps.get("lease").unwrap();
        lease.timestamp = clock.unix_timestamp;
        lease.amount = amount;
        lease.available_withdrawls = 6;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLease<'info> {
    #[account(
        init,
        seeds = [b"lease".as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = size_of::<LeaseAccount>() + 8
    )]
    pub lease: Account<'info, LeaseAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub payer: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LeaseAccount {
    pub signer: Pubkey,
    pub bump: u8,
    pub amount: u64,
    pub timestamp: i64,
    pub available_withdrawls: u8,
}