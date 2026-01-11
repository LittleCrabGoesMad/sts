// 戦闘における必要なUI表示、または処理に必要な情報を提供するモジュール

use crate::entitie::{BattleAscender, Enemy};

pub struct BattleView<'view> {
    pub ascender: &'view BattleAscender,
    pub enemies: &'view[Enemy],
}