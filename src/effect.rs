use crate::battle::Target;

#[allow(unused)]
#[derive(Clone)]
pub enum EffectDef {
    DealDamage { amount: i32 },
    ObtainBlock { amount: i32 },
}

impl EffectDef {
    pub fn to_effect(&self, target: Target) -> Effect {
        match self {
            EffectDef::DealDamage { amount} => Effect::DealDamage { amount: *amount, target: target.clone() },
            EffectDef::ObtainBlock { amount } => Effect::ObtainBlock { amount: *amount, target: target.clone() },
        }
    }
}

#[allow(unused)]
pub enum Effect {
    DealDamage { amount: i32, target: Target },
    ObtainBlock { amount: i32, target: Target },
}
