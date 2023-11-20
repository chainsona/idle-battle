// Idle Battle errors
use anchor_lang::prelude::*;

#[error_code]
pub enum IdleBattleError {
    // Hero is being initialized while already initialized.
    #[msg("Hero already exists")]
    HeroAlreadyExists,

    // The user is trying to do something when the hero is training.
    #[msg("Hero is training; please recall")]
    HeroIsTraining,

    // The user is trying to recall when the hero is not training.
    #[msg("Hero is not training; please train")]
    HeroIsNotTraining,

    // The user is trying to do something while the hero is fighting in a battle.
    #[msg("Battle not over; attack")]
    BattleInProgress,

    // The user is trying to attack when the hero is dead.
    #[msg("No enemy; start battle")]
    BattleNotStarted,

    // The user is trying to attack when the hero is dead.
    #[msg("Hero is dead; respawn to attack")]
    BattleDone,
}
