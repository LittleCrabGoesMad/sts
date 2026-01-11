use crate::effect::EffectDef;
use crate::battle::{BattleContext, EffectResolver, Target};
use crate::entitie::EnemyDef;

pub static CULTIST: EnemyDef = EnemyDef {
    name: "狂信者",
    max_hp: 9,
    enemy_script: cultist_action,
};

pub fn cultist_action(resolver: &EffectResolver, context: &mut BattleContext) {
    // 狂信者の行動: 毎ターン、アセンダーに1ダメージを与える
    println!("狂信者の行動!");
    let target = Target::BattleAscender;
    let damage_effect = EffectDef::DealDamage { amount: 1 };
    resolver.apply(damage_effect.to_effect(target), context);
}