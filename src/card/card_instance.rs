// 戦闘中に使われるカードの内容を規定する
// カードの具体的な実行内容は他ファイルの記述を参照し、インスタンスは単にそこへのアドレスを保持する

use crate::card::{card_def::CardId};
use crate::battle::BattleScript;

#[derive (Debug, Clone)]
pub struct CardInstance {
    pub id: CardId,
    pub name: &'static str,
    pub cost: u8,
    pub card_script: BattleScript,
}