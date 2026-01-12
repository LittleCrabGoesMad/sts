// battle/mod.rs

mod battle_selector;
mod battle_view;
mod battle;
mod effect_resolver;
mod effect;
mod status;

pub use battle::{BattleContext, BattleResult, BattleScript, battle_start};
pub use battle_selector::{BattleSelector, BattleEntity};
pub use effect_resolver::EffectResolver;
pub use effect::{Effect, EffectDef};
pub use status::{StatusDef, Statuses, STRENGTH, VULNERABLE};