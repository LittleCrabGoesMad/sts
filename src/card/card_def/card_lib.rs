use crate::card::{CardDef, card_def::card_preset::*};

// カードのId
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardId {
    Strike,
    Defend,
    Bash,
}

// マスターデッキのカードIDからCardDefを逆検索
pub fn get_card_def(id: CardId) -> &'static CardDef {
    match id {
        CardId::Strike => &STRIKE,
        CardId::Defend => &DEFEND,
        CardId::Bash => &BASH,
    }
}