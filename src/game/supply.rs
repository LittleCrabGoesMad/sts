
// game/supply.rs
use rand::rng;
use rand::seq::SliceRandom;

use crate::card::CardId;

// ゲーム全体の供給物を管理するモジュール
pub struct Supply {
    pub normal_reward_cards: Vec<CardId>,
}

impl Supply {
    // 入力されたアセンダーの定義に基づいて初期化
    pub fn new(ascender_def: &crate::entitie::AscenderDef) -> Self {
        let mut supply = ascender_def.initial_supply.into_supply();
        supply.shuffle_draw_pile();
        supply
    }

    // 報酬カードの山札をシャッフル
    pub fn shuffle_draw_pile(&mut self) {
        let mut rng = rng();
        self.normal_reward_cards.shuffle(&mut rng);
    }
}

// こちらは初期化用の定義
pub struct SupplyDef {
    pub normal_reward_cards: &'static [(CardId, u32)],
}

impl SupplyDef {
    pub fn into_supply(&self) -> Supply {
        let mut normal_reward_cards = Vec::new();

        // 定義に基づいて報酬カードのリストを生成
        for (card_id, count) in self.normal_reward_cards {
            for _ in 0..*count {
                normal_reward_cards.push(*card_id);
            }
        }

        Supply {
            normal_reward_cards,
        }
        // その後シャッフルするのを忘れずに
    }
}