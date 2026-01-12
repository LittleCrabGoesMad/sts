use std::io::{self, Write};

use crate::battle::battle_view::BattleView;
pub struct BattleSelector;

impl BattleSelector {

    fn get_input_number(&self) -> usize {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let trimmed = input.trim();

            // 入力行をうかせるための改行
            println!();

            // 数字以外は弾く
            let Ok(num) = trimmed.parse::<usize>() else {
                println!("半角数字を入力してください");
                continue;
            };

            return num;
        }
    }

    // 敵を１オリジンで選択させ、選択された敵のインデックス(0オリジン)を返す
    pub fn choose_enemy(&self, view: BattleView) -> BattleEntity {
        
        // 敵が１体だけなら自動的にそれを選択する
        if view.enemies.len() == 1 {
            return BattleEntity::Enemy(0);
        }

        loop {
            // 引数に何のための選択なのか示すStringを追加する予定、例えば"攻撃対象を選んでください"など
            println!("\n敵を選択してください:");
            self.show_enemies(&view);
            let num = self.get_input_number();

            if num >= 1 && num <= view.enemies.len() {
                return BattleEntity::Enemy(num - 1);
            }
            println!("1~{}の範囲で入力してください", view.enemies.len());
        }
    }

    // 敵の情報を表示する
    pub fn show_enemies(&self, view: &BattleView) {
        for (i, enemy) in view.enemies.iter().enumerate() {
            println!("{}: {} HP:{}", i + 1, enemy.name, enemy.hp);
        }
    }

    // プレイのためにカードを選択(１オリジン)させ、手札のインデックス(0オリジン)を返す
    // こちらでは0入力によるターン終了を認める
    pub fn choose_card_for_play(&self, view: &BattleView) -> Option<BattleEntity> {
        let deck = &view.ascender.deck;
        // 手札がなければNoneを返す
        if !deck.has_hand() {
            return None;
        }

        // 手札が１枚だけなら自動的にそれを選択する
        if deck.hand.len() == 1 {
            return Some(BattleEntity::Card(0));
        }

        loop {
            println!("\nカードを選択してください (0でターン終了):");
            self.show_hand(&view);
            let num = self.get_input_number();

            // 0が入力されたらターン終了を示すNoneを返す
            if num == 0 {
                return None;
            }

            if num >= 1 && num <= deck.hand.len() {
                return Some(BattleEntity::Card(num - 1));
            }
            println!("0~{}の範囲で入力してください", deck.hand.len());
        }
    }

    // 効果のためにカードを選択(1オリジン)させ、手札のインデックス(0オリジン)を返す
    // こちらでは0入力によるキャンセルは無し
    pub fn choose_card_for_effect(&self, view: &BattleView) -> Option<BattleEntity> {
        let deck = &view.ascender.deck;
        // 手札がなければNoneを返す
        if !deck.has_hand() {
            return None;
        }

        // 手札が１枚だけなら自動的にそれを選択する
        if deck.hand.len() == 1 {
            return Some(BattleEntity::Card(0));
        }

        loop {
            println!("カードを選択してください (0でターン終了):");
            self.show_hand(&view);
            let num = self.get_input_number();

            if num >= 1 && num <= deck.hand.len() {
                return Some(BattleEntity::Card(num - 1));
            }
            println!("1~{}の範囲で入力してください", deck.hand.len());
        }
    }

    // 手札の情報を表示する
    pub fn show_hand(&self, view: &BattleView) {
        for (i, card) in view.ascender.deck.hand.iter().enumerate() {
            println!("{}: {} コスト:{}", i + 1, card.name, card.cost);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BattleEntity {
    BattleAscender,
    Enemy(usize),
    Card(usize),
}