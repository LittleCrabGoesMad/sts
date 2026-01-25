use crate::card::{CardDef, card_def::card_preset::*};

// カードのId
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardId {
    Strike,
    Defend,
    Bash,
    PommelStrike,
    Inflame,
    Hemokinesis,
    Neutralize,
}

// マスターデッキのカードIDからCardDefを逆検索
pub fn get_card_def(id: CardId) -> &'static CardDef {
    match id {
        CardId::Strike => &STRIKE,
        CardId::Defend => &DEFEND,
        CardId::Bash => &BASH,
        CardId::PommelStrike => &POMMEL_STRIKE,
        CardId::Inflame => &INFLAME,
        CardId::Hemokinesis => &HEMOKINESIS,
        CardId::Neutralize => &NEUTRALIZE,
    }
}

// カードIDからカード名を取得する
pub fn get_card_name(id: CardId) -> &'static str {
    get_card_def(id).name
}