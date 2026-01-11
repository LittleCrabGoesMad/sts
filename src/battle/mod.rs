// battle/mod.rs

mod battle_selector;
mod battle_view;
mod battle;
mod effect_resolver;

pub use battle::{BattleContext, BattleResult, BattleScript, battle_start};
pub use battle_selector::{BattleSelector, Target};
pub use effect_resolver::EffectResolver;