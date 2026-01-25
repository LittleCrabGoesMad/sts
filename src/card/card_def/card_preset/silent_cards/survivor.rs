// サバイバー
use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static SURVIVOR: CardDef = CardDef {
    id: CardId::Survivor,
    name: "サバイバー",
    brief_description: "2🛡️ カードを１枚捨てる。",
    cost: 1,
    card_type: CardType::Skill,
    card_script: survivor_play,
};

pub fn survivor_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("サバイバーを発動!");
    let obtain_block = EffectDef::ObtainBlock { amount: 2 };
    resolver.apply(source, obtain_block.to_effect(source), context);
    let discard_card = EffectDef::DiscardCards { amount: 1 };
    resolver.apply(source, discard_card.to_effect(source), context);
}