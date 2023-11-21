//! Instruction: Train
use anchor_lang::prelude::*;

use crate::error::IdleBattleError;
use crate::state::*;

pub fn train(ctx: Context<Train>) -> Result<()> {
    // Check if hero is training
    if ctx.accounts.hero.training_slot > 0 {
        return err!(IdleBattleError::HeroIsTraining);
    }

    // TODO Freeze hero mint

    // Set hero as training
    let slot: u64 = Clock::get()?.slot;
    ctx.accounts.hero.training_slot = slot;
    msg!("Hero is now training");

    Ok(())
}

#[derive(Accounts)]
pub struct Train<'info> {
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
