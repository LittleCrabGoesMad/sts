use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver};
use crate::entitie::EnemyDef;

pub static SMALL_SLIME: EnemyDef = EnemyDef {
    name: "スモールスライム",
    max_hp: 3,
    enemy_script: small_slime_action,
    action_description: "1🗡",
};

pub fn small_slime_action(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    // スモールスライムの行動: 毎ターン、アセンダーに1ダメージを与える
    println!("スモールスライムの行動!");

    let target = CombatantId::Ascender;
    let damage_effect = EffectDef::DealDamage { amount: 1 };
    resolver.apply(source, damage_effect.to_effect(target), context);
}