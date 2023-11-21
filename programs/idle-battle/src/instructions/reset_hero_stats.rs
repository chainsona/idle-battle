//! Instruction: ResetHeroStats
use anchor_lang::prelude::*;

use crate::error::IdleBattleError;
use crate::state::*;

pub fn reset_hero_stats(ctx: Context<ResetStats>) -> Result<()> {
    // Check admin
    let is_admin: bool = ctx.accounts.admin.key() == constants::ADMIN_PUBKEY;
    if !is_admin {
        return err!(IdleBattleError::NotAuthorized);
    }

    // TODO Unfreeze hero mint

    // Set hero as training
    ctx.accounts.hero.training_slot = 0;
    ctx.accounts.hero.xp = 0;
    ctx.accounts.hero.level = 0;
    ctx.accounts.hero.reward = 0;
    msg!("Hero stats reset");

    Ok(())
}

#[derive(Accounts)]
pub struct ResetStats<'info> {
    #[account(address = admin.key())]
    pub admin: Signer<'info>,

    #[account(address = player.key())]
    /// CHECK: The player account.
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [constants::HERO_SEED, player.key.as_ref()],
        bump,
    )]
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
}
