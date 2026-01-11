use crate::{battle::BattleScript, card::CardId};

// カードのスタティックな定義。実体のカードが生成される時、これを参照する。
#[allow(unused)]
pub struct CardDef {
    pub id: CardId,
    pub name: &'static str,
    pub cost: u8,
    pub card_type: CardType,
    pub card_script: BattleScript,
}

// カードのタイプ(アタック、スキル、パワー)
pub enum CardType {
    Attack,
    Skill,
    // Power,
}

//　スクリプト内でユーザーに選択させる対象
#[derive (Debug, Clone, Copy)]
#[allow(unused)]
pub enum TargetSpec {
    NoTarget,
    Enemy,
}