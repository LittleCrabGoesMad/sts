use crate::battle::{BattleContext, EffectResolver, Target};
// 防御
use crate::effect::EffectDef;
use crate::card::card_def::{CardDef, CardId, CardType};

pub static DEFEND :CardDef = CardDef {
    id: CardId::Defend,
    name: "防御",
    cost: 1,
    card_type: CardType::Skill,
    card_script: defend_play,
};

pub fn defend_play(resolver: &EffectResolver, context: &mut BattleContext) {
    println!("防御を発動!");
    let target= Target::BattleAscender;
    let obtain_block = EffectDef::ObtainBlock { amount: 1 };
    resolver.apply(obtain_block.to_effect(target), context);
}