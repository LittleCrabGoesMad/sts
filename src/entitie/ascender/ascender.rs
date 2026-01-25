// アセンダーの共通処理

use std::collections::HashMap;

use crate::battle::Statuses;
use crate::entitie::Combatant;
use crate::game::SupplyDef;
use crate::{BattleResult, battle::BattleScript};
use crate::card::{BattleDeck, CardId, CardInstance, CardType};

#[derive(Clone, Copy)]
pub struct AscenderDef {
    pub name: &'static str,
    pub max_hp: i32,
    pub initial_deck: &'static [(CardId, u32)],
    pub initial_supply: SupplyDef,
}

impl AscenderDef {
    pub fn instantiate(&self) -> Ascender {
        Ascender { 
            name: self.name,
            max_hp: self.max_hp,
            current_hp: self.max_hp,
            master_deck: self.build_master_deck(self.initial_deck), 
        }
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
    // 戦闘用のアセンダーに変換する
    pub fn into_battle(&self) -> BattleAscender {
        let deck = BattleDeck::new_from_master(&self.master_deck);
        BattleAscender {
            name: self.name,
            hp: self.current_hp, 
            max_energy: 3,
            current_energy: 0,
            block: 0,
            statuses: Statuses::new(),
            is_dead: false,
            deck: deck, 
        }
    }

    // マスターデッキにカードを追加する
    pub fn add_card_to_master_deck(&mut self, card_id: CardId) {
        let count = self.master_deck.entry(card_id).or_insert(0);
        *count += 1;
    }
}

pub struct BattleAscender {
    name: &'static str,
    hp: i32,
    max_energy: u8,
    current_energy: u8,
    block: i32,
    statuses: Statuses,
    is_dead: bool,
    pub deck: BattleDeck,
}

impl BattleAscender {
    pub fn get_max_energy(&self) -> u8 {
        self.max_energy
    }

    pub fn get_current_energy(&self) -> u8 {
        self.current_energy
    }

    pub fn out_of_battle(&self) -> BattleResult {
        let result: BattleResult = BattleResult { hp_after: self.hp, victory: !self.is_dead };
        result
    }

    pub fn start_turn(&mut self) {
        self.current_energy = self.max_energy;
        self.block = 0;
        self.deck.draw_some(5);
    }

    pub fn end_turn(&mut self) {
        self.deck.discard_all_hands();
    }

    // カード使用可能かどうか判定する
    pub fn can_use_card(&self, chosen_card_index: usize) -> bool {
        let chosen_card_cost: u8 = self.deck.hand[chosen_card_index].cost;
        self.current_energy >= chosen_card_cost
    }

    // カードを引く
    pub fn draw_cards(&mut self, amount: usize) {
        self.deck.draw_some(amount);
    }

    // カードを捨てる
    pub fn discard_hand(&mut self, target_hand_index: usize) {
        self.deck.discard_hand(target_hand_index);
    }
    // カード使用の共通処理
    pub fn play_card(&mut self, chosen_card_index: usize) -> BattleScript {
        // カードを手札から取り出す
        let card: CardInstance = self.deck.hand.remove(chosen_card_index);
        // エナジー消費
        self.current_energy -= card.cost;
        if card.card_type == CardType::Power {
            // パワーカードは使用済みエリアへ送る
            self.deck.used_power_cards.push(card.clone());
        } else {
            // 捨て札に送る
            self.deck.discard.push(card.clone());
        }
        // カードスクリプトを生成して返す
        card.card_script
    }
}

impl Combatant for BattleAscender {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }
    fn hp_mut(&mut self) -> &mut i32 {
        &mut self.hp
    }

    fn get_block(&self) -> i32 {
        self.block
    }
    fn block_mut(&mut self) -> &mut i32 {
        &mut self.block
    }

    fn is_dead(&self) -> bool {
        self.is_dead
    }
    fn set_dead(&mut self) {
        self.is_dead = true;
    }

    fn get_statuses(&self) -> &Statuses {
        &self.statuses
    }
    fn statuses_mut(&mut self) -> &mut Statuses {
        &mut self.statuses
    }
}