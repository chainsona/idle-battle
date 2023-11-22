//! Instruction: Attack
use anchor_lang::prelude::*;

use crate::error::IdleBattleError;
use crate::state::*;
use crate::utils::*;

pub fn attack(ctx: Context<Attack>) -> Result<()> {
    // Check if fighters have enough health to attack
    if ctx.accounts.battle_round.enemy == 0 {
        return err!(IdleBattleError::BattleNotStarted);
    } else if ctx.accounts.battle_round.hero == 0 {
        return err!(IdleBattleError::BattleDone);
    }

    // Generate pseudo-random number using XORShift with the current slot as seed
    let enemy_seed = Clock::get()?.slot;
    let xorshift_enemy = xorshift64_enemy(enemy_seed);
    msg!("XORShift Enemy: {}", xorshift_enemy);
    // Calculate enemy damage
    let enemy_damage = xorshift_enemy % (constants::ENEMY_MAX_DAMAGE);
    msg!("Enemy Damage: {}", enemy_damage);
    // Subtract enemy from enemy, min health is 0
    ctx.accounts.battle_round.enemy = ctx.accounts.battle_round.enemy.saturating_sub(enemy_damage);
    msg!("Enemy Health: {}", ctx.accounts.battle_round.enemy);

    // Generate pseudo-random number using XORShift with the current slot as seed
    let hero_seed = Clock::get()?.slot;
    let xorshift_hero = xorshift64_hero(hero_seed);
    msg!("XORShift Enemy: {}", xorshift_enemy);
    // Calculate hero damage
    let hero_damage = xorshift_hero % (constants::HERO_MAX_DAMAGE);
    msg!("Hero Damage: {}", hero_damage);
    // Subtract hero from enemy, min health is 0
    ctx.accounts.battle_round.hero = ctx.accounts.battle_round.hero.saturating_sub(hero_damage);
    msg!("Hero Health: {}", ctx.accounts.battle_round.hero);

    Ok(())
}

#[derive(Accounts)]
pub struct Attack<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::BATTLE_SEED],
        bump,
    )]
    pub battle_round: Account<'info, BattleRound>,
}
