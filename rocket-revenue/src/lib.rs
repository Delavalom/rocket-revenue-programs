use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use std::mem::size_of;

declare_id!("6XkEztBwDy4orRcqsBksozKhUN29J1pY87CESxVUB9qK");

#[program]
mod rocket_revenue {
    use super::*;
    pub fn open_lease(ctx: Context<OpenLease>, amount: u64) -> ProgramResult {
        let lease: &mut Account<LeaseAccount> = &mut ctx.accounts.new_lease;
        let signer: &Signer = &ctx.accounts.signer;
        let clock = Clock::get().unwrap();

        lease.signer = *signer.key;
        lease.timestamp = clock.unix_timestamp;
        lease.amount = amount;
        lease.available_withdrawls = 6;
        
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let signer: &Signer = &ctx.accounts.signer;
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
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenLease<'info> {
    #[account(
        init,
        seeds = [b"lease".as_ref(), owner.key().as_ref()],
        bump,
        payer = signer,
        space = size_of::<LeaseAccount>() + 8
    )]
    pub new_lease: Account<'info, LeaseAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct LeaseAccount {
    pub signer: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub available_withdrawls: u8,
}