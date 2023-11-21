//! Instruction: Initialize
use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, TokenAccount};

use crate::error::IdleBattleError;
use crate::state::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // Check admin
    let is_admin: bool = ctx.accounts.admin.key() == constants::ADMIN_PUBKEY;
    if !is_admin {
        return err!(IdleBattleError::NotAuthorized);
    };

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::VAULT_SEED],
        bump,
        payer = admin,
        token::mint = mint,
        token::authority = vault_token_account,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}
