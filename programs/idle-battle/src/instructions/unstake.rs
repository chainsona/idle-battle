//! Instruction: Unstake
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, transfer, Mint, TokenAccount, Transfer};

use crate::error::IdleBattleError;
use crate::state::*;

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(IdleBattleError::InvalidAmount.into());
    }

    if amount > ctx.accounts.stake_token_account.amount {
        return Err(IdleBattleError::InvalidAmount.into());
    }

    let slot: u64 = Clock::get()?.slot;

    ctx.accounts.stake_info.staked_at = slot;

    let staked_amount: u64 = amount
        .checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
        .unwrap();

    let user = ctx.accounts.user.key();
    let bump = *ctx.bumps.get("stake_token_account").unwrap();
    let signer: &[&[&[u8]]] = &[&[constants::STAKE_SEED, user.as_ref(), &[bump]]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.stake_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.stake_token_account.to_account_info(),
            },
            signer,
        ),
        staked_amount,
    )?;
    msg!("User unstaked {} $TOKEN", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::STAKE_INFO_SEED, user.key.as_ref()],
        bump,

    )]
    pub stake_info: Account<'info, StakeInfo>,

    #[account(
        mut,
        seeds = [constants::STAKE_SEED, user.key.as_ref()],
        bump,
    )]
    pub stake_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_account: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
