use crate::entitie::AscenderDef;
use crate::card::CardId;
use crate::game::SupplyDef;

pub static IRONCLAD: AscenderDef = AscenderDef {
    name: "アイアンクラッド",
    max_hp: 10,
    initial_deck: IRONCLAD_INITIAL_MASTER_DECK,
    initial_supply: SupplyDef {
        normal_reward_cards: IRONCLAD_INITIAL_NORMAL_CARD_SUPPLY,
    },
};

static IRONCLAD_INITIAL_MASTER_DECK: &[(CardId, u32)] = &[
    (CardId::Strike, 5),
    (CardId::Defend, 4),
    (CardId::Bash, 1),
];

static IRONCLAD_INITIAL_NORMAL_CARD_SUPPLY: &[(CardId, u32)] = &[
    (CardId::PommelStrike, 1),
    (CardId::Inflame, 1),
    (CardId::Hemokinesis, 1),
];