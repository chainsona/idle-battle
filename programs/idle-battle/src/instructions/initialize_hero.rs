//! Instruction: InitializeHero
use anchor_lang::prelude::*;

use crate::state::*;

pub fn initialize_hero(ctx: Context<InitializeHero>) -> Result<()> {
    // Create the data account linked to a NFT
    ctx.accounts.hero.mint = *ctx.accounts.user.key;
    ctx.accounts.hero.training_slot = 0;
    ctx.accounts.hero.xp = 0;
    ctx.accounts.hero.level = 0;
    ctx.accounts.hero.reward = 0;
    msg!("Hero initialized");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeHero<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::HERO_SEED, user.key.as_ref()],
        space = 8 + std::mem::size_of::<Hero>(),
        bump,
        payer = user,
    )]
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
}
