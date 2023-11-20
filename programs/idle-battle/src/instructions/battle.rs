//! Instruction: Battle
use anchor_lang::prelude::*;

use crate::error::IdleBattleError;
use crate::state::*;

pub fn battle(ctx: Context<Battle>) -> Result<()> {
    // Check if hero is training
    if ctx.accounts.hero.training_slot > 0 {
        return err!(IdleBattleError::HeroIsTraining);
    }

    // Check if fighters have enough health to attack
    if ctx.accounts.battle.enemy > 0 && ctx.accounts.battle.hero > 0 {
        return err!(IdleBattleError::BattleInProgress);
    }

    // Reset enemy to max health
    ctx.accounts.battle.enemy = constants::ENEMY_HEALTH;
    ctx.accounts.battle.hero = constants::HERO_HEALTH;
    Ok(())
}

#[derive(Accounts)]
pub struct Battle<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + std::mem::size_of::<BattleRound>(),
        seeds = [constants::BATTLE_SEED],
        bump,
    )]
    pub battle: Account<'info, BattleRound>,
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
}
