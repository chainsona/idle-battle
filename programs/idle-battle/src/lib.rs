use anchor_lang::prelude::*;

// use clockwork_sdk::state::{Thread, ThreadAccount};

declare_id!("EZhEi1iBCbUyT3tdiMGpye1GVqSzCcSPWeyYxEZ6drn2");

pub mod constants {
    pub const ENEMY_HEALTH: u64 = 1000;
    pub const HERO_HEALTH: u64 = 850;
    pub const ENEMY_MAX_DAMAGE: u64 = 500;
    pub const HERO_MAX_DAMAGE: u64 = 450;

    pub const TRAINING_BASE_XP_TO_LEVEL: u64 = 100;
    pub const TRAINING_XP_GROWTH: f64 = 1.1;
    pub const TRAINING_GOLD_RATE: u64 = 10;

    pub const BATTLE_SEED: &[u8] = b"battle";
    pub const HERO_SEED: &[u8] = b"hero";
}

#[account]
pub struct Hero {
    pub is_initialized: bool,
    pub mint: Pubkey,
    pub training_slot: u64,
    pub xp: u64,
    pub level: u64,
    pub gold: u64,
}

#[account]
pub struct BattleRound {
    pub enemy: u64,
    pub hero: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Hero already exists")]
    HeroAlreadyExists,
    #[msg("Hero is fighting; please runaway")]
    HeroIsFighting,
    #[msg("Hero is training; please recall")]
    HeroIsTraining,
    #[msg("Hero is not training; please train")]
    HeroIsNotTraining,
    #[msg("Battle not over; attack")]
    BattleInProgress,
    #[msg("No enemy; start battle")]
    BattleNotStarted,
    #[msg("Hero is dead; respawn to attack")]
    BattleDone,
}

#[program]
pub mod idle_battle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Check if hero is already initialized
        // if ctx.accounts.hero.is_initialized {
        //     return err!(ErrorCode::HeroAlreadyExists);
        // }

        // Initialize hero
        ctx.accounts.hero.mint = *ctx.accounts.player.key;
        ctx.accounts.hero.training_slot = 0;
        ctx.accounts.hero.xp = 0;
        ctx.accounts.hero.level = 0;
        ctx.accounts.hero.gold = 0;
        msg!("Hero initialized");

        Ok(())
    }

    pub fn train(ctx: Context<Train>) -> Result<()> {
        // Check if hero is training
        if ctx.accounts.hero.training_slot > 0 {
            return err!(ErrorCode::HeroIsTraining);
        }

        // TODO Freeze hero mint

        // Set hero as training
        let slot: u64 = Clock::get()?.slot;
        ctx.accounts.hero.training_slot = slot;
        msg!("Hero is now training");

        Ok(())
    }

    pub fn recall(ctx: Context<Recall>) -> Result<()> {
        // Check if hero is training
        if ctx.accounts.hero.training_slot == 0 {
            return err!(ErrorCode::HeroIsNotTraining);
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

    pub fn resetstats(ctx: Context<ResetStats>) -> Result<()> {
        // TODO Unfreeze hero mint

        // Set hero as training
        ctx.accounts.hero.training_slot = 0;
        ctx.accounts.hero.xp = 0;
        ctx.accounts.hero.level = 0;
        ctx.accounts.hero.gold = 0;
        msg!("Hero stats reset");

        Ok(())
    }

    pub fn battle(ctx: Context<Battle>) -> Result<()> {
        // Check if hero is training
        if ctx.accounts.hero.training_slot > 0 {
            return err!(ErrorCode::HeroIsTraining);
        }

        // Check if fighters have enough health to attack
        if ctx.accounts.battle.enemy > 0 && ctx.accounts.battle.hero > 0 {
            return err!(ErrorCode::BattleInProgress);
        }

        // Reset enemy to max health
        ctx.accounts.battle.enemy = constants::ENEMY_HEALTH;
        ctx.accounts.battle.hero = constants::HERO_HEALTH;
        Ok(())
    }

    pub fn runaway(ctx: Context<Battle>) -> Result<()> {
        // Check if fighters have enough health to attack
        if ctx.accounts.battle.enemy > 0 && ctx.accounts.battle.hero > 0 {
            return err!(ErrorCode::BattleNotStarted);
        }

        // Reset enemy to max health
        ctx.accounts.battle.enemy = constants::ENEMY_HEALTH;
        ctx.accounts.battle.hero = constants::HERO_HEALTH;
        Ok(())
    }

    pub fn attack(ctx: Context<Attack>) -> Result<()> {
        // Check if fighters have enough health to attack
        if ctx.accounts.battle_round.enemy == 0 {
            return err!(ErrorCode::BattleNotStarted);
        } else if ctx.accounts.battle_round.hero == 0 {
            return err!(ErrorCode::BattleDone);
        }

        // Generate pseudo-random number using XORShift with the current slot as seed
        let enemy_seed = Clock::get()?.slot;
        let xorshift_enemy = xorshift64_enemy(enemy_seed);
        msg!("XORShift Enemy: {}", xorshift_enemy);
        // Calculate enemy damage
        let enemy_damage = xorshift_enemy % (constants::ENEMY_MAX_DAMAGE);
        msg!("Enemy Damage: {}", enemy_damage);
        // Subtract enemy from enemy, min health is 0
        ctx.accounts.battle_round.enemy =
            ctx.accounts.battle_round.enemy.saturating_sub(enemy_damage);
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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + std::mem::size_of::<Hero>(),
        seeds = [constants::HERO_SEED, player.key.as_ref()],
        bump,
    )]
    pub hero: Account<'info, Hero>,
    pub system_program: Program<'info, System>,
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

#[derive(Accounts)]
pub struct Attack<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [constants::BATTLE_SEED],
        bump,
    )]
    pub battle_round: Account<'info, BattleRound>,
}

pub fn xorshift64_enemy(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn xorshift64_hero(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 17;
    x ^= x >> 13;
    x ^= x << 7;
    x
}
