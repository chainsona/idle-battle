//! Idle Battle program instructions
pub mod attack;
pub mod battle;
pub mod initialize;
pub mod initialize_hero;
pub mod recall;
pub mod reset_hero_stats;
pub mod runaway;
pub mod train;

pub use attack::*;
pub use battle::*;
pub use initialize::*;
pub use initialize_hero::*;
pub use recall::*;
pub use reset_hero_stats::*;
pub use runaway::*;
pub use train::*;
