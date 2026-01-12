// 防御
use crate::battle::{BattleContext, EffectDef, EffectResolver, BattleEntity};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static DEFEND :CardDef = CardDef {
    id: CardId::Defend,
    name: "防御",
    cost: 1,
    card_type: CardType::Skill,
    card_script: defend_play,
};

pub fn defend_play(source: &BattleEntity, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("防御を発動!");
    let target= BattleEntity::BattleAscender;
    let obtain_block = EffectDef::ObtainBlock { amount: 1 };
    resolver.apply(*source, obtain_block.to_effect(target), context);
}