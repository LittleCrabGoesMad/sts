// 山札や捨て札、手札といった、戦闘中のデッキ処理を管理する
use std::collections::HashMap;
use rand::rng;
use rand::seq::SliceRandom;

use crate::card::{CardDef, CardInstance, CardId, get_card_def};

pub struct BattleDeck {
    pub draw_pile: Vec<CardInstance>,
    pub hand: Vec<CardInstance>,
    pub discard: Vec<CardInstance>,
    // pub exhausted: Vec<CardInstance>,
    pub used_power_cards: Vec<CardInstance>,
}

impl BattleDeck {
    // マスターデッキから生成
    pub fn new_from_master(master: &HashMap<CardId, u32>) -> Self {
        let mut draw_pile = Vec::new();

        for (card_id, count) in master {
            let card_def: &CardDef = get_card_def(*card_id);

            for _ in 0..*count {
                draw_pile.push(CardInstance {
                    id: *card_id,
                    name: card_def.name,
                    brief_description: card_def.brief_description,
                    cost: card_def.cost,
                    card_type: card_def.card_type,
                    card_script: card_def.card_script.clone(),
                });
            }
        }

        let mut deck: BattleDeck = BattleDeck { 
            draw_pile: draw_pile,
            hand: Vec::new(),
            discard: Vec::new(),
            used_power_cards: Vec::new(),
        };

        // 必ずシャッフルした状態で渡すこと
        deck.shuffle_draw_pile();
        deck
    }

    // 複数回シャッフル
    pub fn draw_some(&mut self, amount: usize) {
        for _ in 0..amount {
            self.draw();
        }
    }

    // ドロー処理
    fn draw(&mut self) {
       if let Some(card)  = self.draw_pile.pop() {
            self.hand.push(card);
       } else {
            self.reshuffle();
            if let Some(card)  = self.draw_pile.pop() {
                self.hand.push(card);
            }
       }
    }

    // 山札をシャッフル
    pub fn shuffle_draw_pile(&mut self) {
        let mut rng = rng();
        self.draw_pile.shuffle(&mut rng);
    }

    pub fn has_hand(&self) -> bool {
        self.hand.len() > 0
    }
    
    // 手札を捨て札に全て送る
    pub fn discard_hand(&mut self) {
        while !self.hand.is_empty() {
            let card: CardInstance = self.hand.remove(0);
            self.discard.push(card);
        }
    }

    // 捨て札を山札に全て送り、シャッフルする
    fn reshuffle(&mut self) {
        println!("リシャッフル！");
        while !self.discard.is_empty() {
            let card: CardInstance = self.discard.remove(0);
            self.draw_pile.push(card);
        }
        self.shuffle_draw_pile();
    }
}