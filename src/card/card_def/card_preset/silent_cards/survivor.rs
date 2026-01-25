// サバイバー

use crate::battle::{BattleContext, CombatantId, Effect, EffectResolver};
use crate::card::card_def::{CardDef, CardId, CardType};

pub static SURVIVOR :CardDef = CardDef {
    id: CardId::Survivor,
    name: "サバイバー",
    brief_description: "2🛡, 手札から任意の1枚を捨てる。",
    cost: 1,
    card_type: CardType::Skill,
    card_script: survivor_play,
};

pub fn survivor_play(source: CombatantId, resolver: &EffectResolver, context: &mut BattleContext) {
    println!("サバイバーを発動!");
    let target= CombatantId::Ascender;
    let obtain_block = Effect::ObtainBlock { amount: 2, target };
    resolver.apply(source, obtain_block, context);

    if let Some(target_hand_index)  
    = context.battle_selector.choose_card_for_effect(context.create_view(), "捨てるカードを選択") {
        let discard_card = Effect::Discard { target_hand_index };
        resolver.apply(source, discard_card, context);
    }
}