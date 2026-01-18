// 発火
use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver, STRENGTH};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static INFLAME :CardDef = CardDef {
    id: CardId::Inflame,
    name: "発火",
    brief_description: "💪",
    cost: 2,
    card_type: CardType::Power,
    card_script: inflame_play,
};

pub fn inflame_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("発火を発動!");
    let target = source; // 自分自身に強化を付与
    let strength = EffectDef::ApplyStatus { status_def: STRENGTH.clone(), amount: 1 };
    resolver.apply(source, strength.to_effect(target), context);
}