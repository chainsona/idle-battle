//! Instruction: ResetStats
use anchor_lang::prelude::*;

use crate::state::*;

pub fn reset_hero_stats(ctx: Context<ResetStats>) -> Result<()> {
    // TODO Unfreeze hero mint

    // Set hero as training
    ctx.accounts.hero.training_slot = 0;
    ctx.accounts.hero.xp = 0;
    ctx.accounts.hero.level = 0;
    ctx.accounts.hero.gold = 0;
    msg!("Hero stats reset");

    Ok(())
}

#[derive(Accounts)]
pub struct ResetStats<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::HERO_SEED, player.key.as_ref()],
        bump,
    )]
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
}
