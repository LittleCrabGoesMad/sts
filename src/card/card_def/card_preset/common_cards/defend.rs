// 防御
use crate::battle::{BattleContext, CombatantId, Effect, EffectResolver};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static DEFEND :CardDef = CardDef {
    id: CardId::Defend,
    name: "防御",
    brief_description: "1🛡",
    cost: 1,
    card_type: CardType::Skill,
    card_script: defend_play,
};

pub fn defend_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("防御を発動!");
    let target= CombatantId::Ascender;
    let obtain_block = Effect::ObtainBlock { amount: 1, target };
    resolver.apply(source, obtain_block, context);
}