use crate::battle::{StatusDef, CombatantId};

#[allow(unused)]
#[derive(Clone)]
pub enum EffectDef {
    DealDamage { amount: i32 },
    ObtainBlock { amount: i32 },
    ApplyStatus { status_def: StatusDef, amount: i32 },
    DrawCards { amount: i32 },
}

impl EffectDef {
    pub fn to_effect(&self, target: CombatantId) -> Effect {
        match self {
            EffectDef::DealDamage { amount} => Effect::DealDamage { amount: *amount, target: target.clone() },
            EffectDef::ObtainBlock { amount } => Effect::ObtainBlock { amount: *amount, target: target.clone() },
            EffectDef::ApplyStatus { status_def, amount } => Effect::ApplyStatus { status_def: status_def.clone(), amount: *amount, target: target.clone() },
            EffectDef::DrawCards { amount } => Effect::DrawCards { amount: *amount, target: target.clone() },
        }
    }
}

#[allow(unused)]
pub enum Effect {
    DealDamage { amount: i32, target: CombatantId },
    ObtainBlock { amount: i32, target: CombatantId },
    ApplyStatus { status_def: StatusDef, amount: i32, target: CombatantId },
    DrawCards { amount: i32, target: CombatantId },
}