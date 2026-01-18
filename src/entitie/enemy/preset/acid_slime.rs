use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver};
use crate::entitie::EnemyDef;

pub static ACID_SLIME: EnemyDef = EnemyDef {
    name: "アシッドスライム",
    max_hp: 5,
    enemy_script: acid_slime_action,
    action_description: "2🗡",
};

pub fn acid_slime_action(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    // アシッドスライムの行動: 毎ターン、アセンダーに2ダメージを与える
    println!("アシッドスライムの行動!");

    let target = CombatantId::Ascender;
    let damage_effect = EffectDef::DealDamage { amount: 2 };
    resolver.apply(source, damage_effect.to_effect(target), context);
}