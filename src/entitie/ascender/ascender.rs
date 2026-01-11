// アセンダーの共通処理

use std::collections::HashMap;

use crate::{BattleResult, battle::BattleScript};
use crate::card::{BattleDeck, CardId, CardInstance, };

pub struct AscenderDef {
    pub name: &'static str,
    pub max_hp: i32,
    pub initial_deck: &'static [(CardId, u32)],
}

impl AscenderDef {
    pub fn instantiate(&self) -> Ascender {
        Ascender { 
            name: self.name,
            max_hp: self.max_hp,
            current_hp: self.max_hp,
            master_deck: self.build_master_deck(self.initial_deck), }
    }

    fn build_master_deck(&self, initial_deck: &[(CardId, u32)]) -> HashMap<CardId, u32> {
        let mut master_deck = HashMap::new();

        for (card_id, count) in initial_deck {
            master_deck.insert(*card_id, *count);
        }
        master_deck
    }
}
#[allow(unused)]
pub struct Ascender {
    pub name: &'static str,
    pub max_hp :i32,
    pub current_hp :i32,
    pub master_deck: HashMap<CardId, u32>,
}

impl Ascender {
    pub fn into_battle(&self) -> BattleAscender {
        let deck = BattleDeck::new_from_master(&self.master_deck);

        BattleAscender {
            name: self.name,
            hp: self.current_hp, 
            max_energy: 3,
            energy: 0,
            block: 0,
            is_dead: false,
            deck: deck, 
        }
    }
}

pub struct BattleAscender {
    name: &'static str,
    hp: i32,
    max_energy: u8,
    energy: u8,
    block: i32,
    pub is_dead: bool,
    pub deck: BattleDeck,
}

impl BattleAscender {
    pub fn out_of_battle(&self) -> BattleResult {
        let result: BattleResult = BattleResult { hp_after: self.hp, victory: !self.is_dead };
        result
    }

    pub fn start_turn(&mut self) {
        self.energy = self.max_energy;
        self.block = 0;
        self.deck.draw_some(5);
    }

    pub fn end_turn(&mut self) {
        self.deck.discard_hand();
    }
    
    pub fn take_damage(&mut self, amount: &i32) {
        let mut damage = *amount;
        if self.block >= damage {
            // ブロックが上回るか同等の場合ブロック値を差し引いて終わり
            self.block -= damage;
            println!("攻撃を防ぎ切った！");
            println!("{}のブロック:{}", self.name, self.block);
            return;
        }
        damage -= self.block;
        self.block = 0;
        self.hp -= damage;
        println!("{}ダメージを受けてしまった", damage);
        if self.hp <= 0 {
            self.is_dead = true;
            println!("死んでしまった！！！");
        }
    }

    pub fn obtain_block(&mut self, amount: &i32) {
        println!("ブロックを{}獲得した", amount);
        self.block += amount;
    }

    // カード使用可能かどうか判定する
    pub fn can_use_card(&self, chosen_card_index: usize) -> bool {
        let chosen_card_cost: u8 = self.deck.hand[chosen_card_index].cost;
        self.energy >= chosen_card_cost
    }

    // カード使用の共通処理
    pub fn play_card(&mut self, chosen_card_index: usize) -> BattleScript {
        // カードを手札から取り出す
        let card: CardInstance = self.deck.hand.remove(chosen_card_index);
        // エナジー消費
        self.energy -= card.cost;
        // 捨て札に送る
        self.deck.discard.push(card.clone());
        // カードスクリプトを生成して返す
        card.card_script
    }
}
