//! Instruction: Recall
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, transfer, Mint, TokenAccount, Transfer};

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
    let slot_passed: u64 = slot - ctx.accounts.hero.training_slot;
    let gold_gained: u64 = (constants::TRAINING_GOLD_RATE * slot_passed)
        .checked_mul(10u64.pow(ctx.accounts.mint_account.decimals as u32))
        .unwrap();
    if ctx.accounts.hero.level > 1 {
        ctx.accounts.hero.gold += gold_gained;

        // TODO Transfer earned $TOKEN to player
        let bump = *ctx.bumps.get("vault_token_account").unwrap();
        let signer: &[&[&[u8]]] = &[&[constants::VAULT_SEED, &[bump]]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.player_token_account.to_account_info(),
                    authority: ctx.accounts.vault_token_account.to_account_info(),
                },
                signer,
            ),
            gold_gained,
        )?;
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

    #[account(
        mut,
        seeds = [constants::VAULT_SEED],
        bump,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = player,
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    pub mint_account: Account<'info, Mint>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_account: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
