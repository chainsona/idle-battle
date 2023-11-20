//! Instruction: Runaway
use anchor_lang::prelude::*;

use crate::error::IdleBattleError;
use crate::state::*;

pub fn runaway(ctx: Context<Runaway>) -> Result<()> {
    // Check if fighters have enough health to attack
    if ctx.accounts.battle.enemy > 0 && ctx.accounts.battle.hero > 0 {
        return err!(IdleBattleError::BattleNotStarted);
    }

    // Reset enemy to max health
    ctx.accounts.battle.enemy = constants::ENEMY_HEALTH;
    ctx.accounts.battle.hero = constants::HERO_HEALTH;
    Ok(())
}

#[derive(Accounts)]
pub struct Runaway<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::BATTLE_SEED],
        bump,
    )]
    pub battle: Account<'info, BattleRound>,
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
}
