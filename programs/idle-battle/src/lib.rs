pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;
use instructions::*;

// use clockwork_sdk::state::{Thread, ThreadAccount};

declare_id!("EZhEi1iBCbUyT3tdiMGpye1GVqSzCcSPWeyYxEZ6drn2");

#[program]
pub mod idle_battle {
    use super::*;

    /// Initialize the player account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// Initialize the hero state account
    pub fn initialize_hero(ctx: Context<InitializeHero>) -> Result<()> {
        instructions::initialize_hero(ctx)
    }

    /// Train the hero
    pub fn train(ctx: Context<Train>) -> Result<()> {
        instructions::train(ctx)
    }

    /// Recall the hero from training
    pub fn recall(ctx: Context<Recall>) -> Result<()> {
        instructions::recall(ctx)
    }

    /// Reset the hero stats
    pub fn reset_hero_stats(ctx: Context<ResetStats>) -> Result<()> {
        instructions::reset_hero_stats(ctx)
    }

    /// Stake an amount of $TOKEN
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    /// Reset the hero stats
    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::unstake(ctx, amount)
    }

    /// Start a battle
    pub fn battle(ctx: Context<Battle>) -> Result<()> {
        instructions::battle(ctx)
    }

    /// Attack the enemy
    pub fn attack(ctx: Context<Attack>) -> Result<()> {
        instructions::attack(ctx)
    }

    /// Runaway from a battle
    pub fn runaway(ctx: Context<Runaway>) -> Result<()> {
        instructions::runaway(ctx)
    }
}
