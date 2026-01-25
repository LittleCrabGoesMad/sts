// ヘモキネシス
use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static HEMOKINESIS :CardDef = CardDef {
    id: CardId::Hemokinesis,
    name: "ヘモキネシス",
    brief_description: "自身に🗡1, 3🗡️",
    cost: 1,
    card_type: CardType::Attack,
    card_script: hemokinesis_play,
};

pub fn hemokinesis_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("ヘモキネシスを発動!");
    let self_damage = EffectDef::DealDamage { amount: 1 };
    resolver.apply(source, self_damage.to_effect(source), context);

    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let damage = EffectDef::DealDamage { amount: 3 };
    resolver.apply(source, damage.to_effect(target), context);
} 