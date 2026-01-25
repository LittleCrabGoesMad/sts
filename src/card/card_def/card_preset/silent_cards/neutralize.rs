// 無力化

use crate::battle::{BattleContext, CombatantId, Effect, EffectResolver, WEAK};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static NEUTRALIZE: CardDef = CardDef {
    id: CardId::Neutralize,
    name: "無力化",
    brief_description: "1🗡️, 脱力を与える",
    cost: 0,
    card_type: CardType::Attack,
    card_script: neutralize_play,
};
    
pub fn neutralize_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("無力化を発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = Effect::DealDamage { amount: 1, target };
    resolver.apply(source, deal_damage, context);
    let apply_weak = Effect::ApplyStatus { status_def: WEAK, amount: 1, target };
    resolver.apply(source, apply_weak, context);
}