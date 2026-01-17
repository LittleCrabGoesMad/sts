// 強打
use crate::battle::{BattleContext, EffectDef, EffectResolver,CombatantId, VULNERABLE};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static BASH :CardDef = CardDef {
    id: CardId::Bash,
    name: "強打",
    cost: 2,
    card_type: CardType::Attack,
    card_script: bash_play,
};

pub fn bash_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("強打を発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = EffectDef::DealDamage { amount: 2 };
    resolver.apply(source, deal_damage.to_effect(target), context);
    let apply_vulnerable = EffectDef::ApplyStatus { status_def: VULNERABLE, amount: 1 };
    resolver.apply(source, apply_vulnerable.to_effect(target), context);
}