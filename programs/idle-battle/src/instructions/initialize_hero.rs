//! Instruction: Initialize
use anchor_lang::prelude::*;

use crate::state::*;

pub fn initialize_hero(ctx: Context<InitializeHero>) -> Result<()> {
    // Initialize hero
    ctx.accounts.hero.mint = *ctx.accounts.player.key;
    ctx.accounts.hero.training_slot = 0;
    ctx.accounts.hero.xp = 0;
    ctx.accounts.hero.level = 0;
    ctx.accounts.hero.gold = 0;
    msg!("Hero initialized");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeHero<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + std::mem::size_of::<Hero>(),
        seeds = [constants::HERO_SEED, player.key.as_ref()],
        bump,
    )]
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
}
