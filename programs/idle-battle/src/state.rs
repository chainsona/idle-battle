// Idle Battle account state

use anchor_lang::prelude::*;

pub mod constants {
    use solana_program::{pubkey, pubkey::Pubkey};

    pub const ENEMY_HEALTH: u64 = 1000;
    pub const HERO_HEALTH: u64 = 850;
    pub const ENEMY_MAX_DAMAGE: u64 = 500;
    pub const HERO_MAX_DAMAGE: u64 = 450;

    pub const TRAINING_BASE_XP_TO_LEVEL: u64 = 100;
    pub const TRAINING_XP_GROWTH: f64 = 1.1;
    pub const TRAINING_GOLD_RATE: u64 = 10;

    pub const BATTLE_SEED: &[u8] = b"battle";
    pub const HERO_SEED: &[u8] = b"hero";
    pub const TOKEN_SEED: &[u8] = b"token";
    pub const VAULT_SEED: &[u8] = b"vault";

    pub const ADMIN_PUBKEY: Pubkey = pubkey!("Acx9SmHnz2GRQYey7TSeKPXdTSG52MsLGn7S2FyXCVE9");
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
