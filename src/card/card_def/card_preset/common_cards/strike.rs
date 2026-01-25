// ストライク
use crate::battle::{BattleContext, CombatantId, Effect, EffectResolver};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static STRIKE: CardDef = CardDef {
    id: CardId::Strike,
    name: "ストライク",
    brief_description: "1🗡️",
    cost: 1,
    card_type: CardType::Attack,
    card_script: strike_play,
};

pub fn strike_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("ストライクを発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = Effect::DealDamage { amount: 1, target };
    resolver.apply(source, deal_damage, context);
}