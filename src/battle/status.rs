// battle/status.rs
// 戦闘中のステータス効果を規定する

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusDef {
    pub name: &'static str,         // 名前
    // pub description: &'static str,  // 説明
    // pub stack_type: StackType,        // スタックの種類 (加算するか、上書きするか、ゼロを認めるかなど）
    pub is_positive: bool,          // ポジティブ効果かどうか
}

// スタックの種類
/* #[derive(Debug, Clone)]
pub enum StackType {
    Additive,    */


// ステータス効果とそのスタック数のマップ
pub struct Statuses {
    stacks: HashMap<StatusDef, i32>,
}

// ステータス効果の管理
impl Statuses {
    // コンストラクタ
    pub fn new() -> Self {
        Self { stacks: HashMap::new() }
    }
    // ステータス効果を追加する
    // 既に存在する場合はスタック数を増加させ、なければ新規に追加する
    pub fn insert(&mut self, status_def: &StatusDef, amount: i32) {
        self.stacks.entry(*status_def)
            .and_modify(|val| *val += amount)
            .or_insert(amount);
    }

    // ステータス効果のスタック数を取得する
    pub fn get(&self, status_def: &StatusDef) -> i32 {
        *self.stacks.get(status_def).unwrap_or(&0)
    }

    // ステータス効果を消費する
    pub fn can_consume(&mut self, status_def: &StatusDef, amount: i32) -> bool {
        if let Some(val) = self.stacks.get_mut(status_def) {
            if *val < amount {
                // スタック数が足りない場合
                return false;
            }
            *val -= amount;
            if *val == 0 {
                // スタック数が0になったら削除する
                self.stacks.remove(status_def);
            }
            return true;
        }
        // ステータスを持たない場合
        false
    }
}

// 筋力：　与えるダメージが増加する
pub static STRENGTH: StatusDef = StatusDef {
    name: "筋力",
    // description: "Increases damage dealt.",
    is_positive: true,
};

// 弱体化: 受けるダメージが増加する
pub static VULNERABLE: StatusDef = StatusDef {
    name: "弱体化",
    // description: "Increases damage taken.",
    is_positive: false,
};