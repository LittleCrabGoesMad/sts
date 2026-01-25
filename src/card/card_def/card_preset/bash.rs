// 強打
use crate::battle::{BattleContext, Effect, EffectResolver,CombatantId, VULNERABLE};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static BASH :CardDef = CardDef {
    id: CardId::Bash,
    name: "強打",
    brief_description: "2🗡️ 💔",
    cost: 2,
    card_type: CardType::Attack,
    card_script: bash_play,
};

pub fn bash_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("強打を発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = Effect::DealDamage { amount: 2, target };
    resolver.apply(source, deal_damage, context);
    let apply_vulnerable = Effect::ApplyStatus { status_def: VULNERABLE, amount: 1, target };
    resolver.apply(source, apply_vulnerable, context);
}