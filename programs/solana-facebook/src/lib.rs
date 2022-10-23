use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Transfer};
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("arcT3EprvPhk1o8Ss1sTyUBwVAckDt6bks2P4ngpSJN");

#[program]
mod rocket_revenue {
    use super::*;
    pub fn transfer_token(ctx: Context<TransferToken>) -> ProgramResult {
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        anchor_spl::token::transfer(cpi_ctx, 5)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// Check: this is not dangeuros cause we don't read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
}