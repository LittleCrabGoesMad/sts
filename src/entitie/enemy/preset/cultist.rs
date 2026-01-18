use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver, STRENGTH};
use crate::entitie::EnemyDef;

pub static CULTIST: EnemyDef = EnemyDef {
    name: "狂信者",
    max_hp: 9,
    enemy_script: cultist_action,
    action_description: "1🗡  💪",
};

pub fn cultist_action(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    // 狂信者の行動: 毎ターン、アセンダーに1ダメージを与え、自身に1の筋力を付与する
    println!("狂信者の行動!");

    let target = CombatantId::Ascender;
    let damage_effect = EffectDef::DealDamage { amount: 1 };
    resolver.apply(source, damage_effect.to_effect(target), context);

    let target = source; // 自分自身に強化を付与
    let strength = EffectDef::ApplyStatus { status_def: STRENGTH.clone(), amount: 1 };
    resolver.apply(source, strength.to_effect(target), context);
}