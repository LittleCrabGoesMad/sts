use crate::battle::{BattleContext, EffectResolver};
// ストライク
use crate::effect::EffectDef;
use crate::card::card_def::{CardDef, CardId, CardType};

pub static STRIKE: CardDef = CardDef {
    id: CardId::Strike,
    name: "ストライク",
    cost: 1,
    card_type: CardType::Attack,
    card_script: strike_play,
};

pub fn strike_play(resolver: &EffectResolver, context: &mut BattleContext) {
    println!("ストライクを発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = EffectDef::DealDamage { amount: 1 };
    resolver.apply(deal_damage.to_effect(target), context);
}