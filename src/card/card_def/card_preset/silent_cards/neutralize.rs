// 無力化
use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver, WEAK};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static NEUTRALIZE: CardDef = CardDef {
    id: CardId::Neutralize,
    name: "無力化",
    brief_description: "1🗡️ 1脱力",
    cost: 0,
    card_type: CardType::Attack,
    card_script: neutralize_play,
};

pub fn neutralize_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("無力化を発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = EffectDef::DealDamage { amount: 1 };
    resolver.apply(source, deal_damage.to_effect(target), context);
    let apply_weak = EffectDef::ApplyStatus { status_def: WEAK, amount: 1 };
    resolver.apply(source, apply_weak.to_effect(target), context);
}