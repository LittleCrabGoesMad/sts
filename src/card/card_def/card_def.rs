use crate::{battle::BattleScript, card::CardId};

// カードのスタティックな定義。実体のカードが生成される時、これを参照する。
#[allow(unused)]
pub struct CardDef {
    pub id: CardId,
    pub name: &'static str,
    pub brief_description: &'static str,
    pub cost: u8,
    pub card_type: CardType,
    pub card_script: BattleScript,
}

// カードのタイプ(アタック、スキル、パワー)
#[derive (Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Attack,
    Skill,
    Power,
}