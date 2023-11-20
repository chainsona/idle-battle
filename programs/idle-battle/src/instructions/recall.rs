//! Instruction: Recall
use anchor_lang::prelude::*;

use crate::error::IdleBattleError;
use crate::state::*;

pub fn recall(ctx: Context<Recall>) -> Result<()> {
    // Check if hero is training
    if ctx.accounts.hero.training_slot == 0 {
        return err!(IdleBattleError::HeroIsNotTraining);
    }

    // TODO Unfreeze hero mint

    // TODO Automate XP growth with clockwork

    // Add XP to hero based on time spent training
    let slot: u64 = Clock::get()?.slot;
    let xp_gained: u64 =
        constants::TRAINING_BASE_XP_TO_LEVEL * (slot - ctx.accounts.hero.training_slot);
    ctx.accounts.hero.xp += xp_gained;

    // Level up hero if XP is enough
    let xp_required: f32 = constants::TRAINING_BASE_XP_TO_LEVEL as f32
        * f32::powf(
            constants::TRAINING_XP_GROWTH as f32,
            ctx.accounts.hero.level as f32,
        );
    ctx.accounts.hero.level = ctx.accounts.hero.xp / xp_required as u64;

    // Add gold to hero based on time spent training
    if ctx.accounts.hero.level > 1 {
        let gold_gained: u64 =
            constants::TRAINING_GOLD_RATE * (slot - ctx.accounts.hero.training_slot);
        ctx.accounts.hero.gold += gold_gained;
    }

    // Set hero as not training
    ctx.accounts.hero.training_slot = 0;
    msg!("Hero came back from training");

    Ok(())
}

#[derive(Accounts)]
pub struct Recall<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::HERO_SEED, player.key.as_ref()],
        bump,
    )]
    pub hero: Account<'info, Hero>,
}
