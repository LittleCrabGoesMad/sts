use crate::entitie::AscenderDef;
use crate::card::CardId;
use crate::game::SupplyDef;

pub static SILENT: AscenderDef = AscenderDef {
    name: "サイレント",
    max_hp: 10,
    initial_deck: SILENT_INITIAL_MASTER_DECK,
    initial_supply: SupplyDef {
        normal_reward_cards: SILENT_INITIAL_NORMAL_CARD_SUPPLY,
    },
};

static SILENT_INITIAL_MASTER_DECK: &[(CardId, u32)] = &[
    (CardId::Strike, 5),
    (CardId::Defend, 4),
    (CardId::Neutralize, 1),
    (CardId::Survivor, 1),
];

static SILENT_INITIAL_NORMAL_CARD_SUPPLY: &[(CardId, u32)] = &[
    (CardId::PommelStrike, 1),
    (CardId::Inflame, 1),
    (CardId::Hemokinesis, 1),
];