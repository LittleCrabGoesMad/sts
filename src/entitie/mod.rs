// entities/mod.rs
pub mod ascender;
pub mod enemy;
pub mod conbatant;

pub use ascender::{AscenderDef, Ascender, BattleAscender};
pub use enemy::{EnemyDef, Enemy};
pub use conbatant::Combatant;