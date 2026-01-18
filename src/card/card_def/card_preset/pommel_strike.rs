// ポンメルストライク
use crate::battle::{BattleContext, CombatantId, EffectDef, EffectResolver};
use crate::card::card_def::{CardDef, CardId, CardType};
pub static POMMEL_STRIKE: CardDef = CardDef {
    id: CardId::PommelStrike,
    name: "ポンメルストライク",
    brief_description: "2🗡️ カードを1枚引く。",
    cost: 1,
    card_type: CardType::Attack,
    card_script: pommel_strike_play,
};

pub fn pommel_strike_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("ポンメルストライクを発動!");
    let target= {
        context.battle_selector.choose_enemy(context.create_view())
    };
    let deal_damage = EffectDef::DealDamage { amount: 2 };
    resolver.apply(source, deal_damage.to_effect(target), context);
    let draw_card = EffectDef::DrawCards { amount: 1 };
    resolver.apply(source, draw_card.to_effect(source), context);
}