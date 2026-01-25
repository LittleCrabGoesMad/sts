use crate::battle::{StatusDef, CombatantId};

#[allow(unused)]
pub enum Effect {
    DealDamage { amount: i32, target: CombatantId },
    ObtainBlock { amount: i32, target: CombatantId },
    ApplyStatus { status_def: StatusDef, amount: i32, target: CombatantId },
    DrawCards { amount: i32, target: CombatantId },
}