use crate::entitie::AscenderDef;
use crate::card::CardId;

pub static IRONCLAD: AscenderDef = AscenderDef {
    name: "アイアンクラッド",
    max_hp: 10,
    initial_deck: IRONCLAD_INITIAL_MASTER_DECK,
};

static IRONCLAD_INITIAL_MASTER_DECK: &[(CardId, u32)] = &[
    (CardId::Strike, 5),
    (CardId::Defend, 4),
    (CardId::Bash, 1),
];