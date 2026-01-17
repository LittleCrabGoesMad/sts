// entitie/conbatant.rs
// 戦闘に参加する存在のトレイトを定義するモジュール

use crate::battle::{StatusDef, Statuses};

pub trait Combatant {
    fn get_name(&self) -> &str;

    fn get_hp(&self) -> i32;
    fn hp_mut(&mut self) -> &mut i32;

    fn get_block(&self) -> i32;
    fn block_mut(&mut self) -> &mut i32;

    fn is_dead(&self) -> bool;
    fn set_dead(&mut self);

    fn take_damage(&mut self, amount: &i32) {
        let mut damage = *amount;

        if self.get_block() >= damage {
            // ブロックを先に消費
            *self.block_mut() -= damage;
            println!("攻撃を防ぎ切った！");
            println!("{}のブロック:{}", self.get_name(), self.get_block());
            return;
        } else {
            // ブロックを全て消費してから残りのダメージをHPに適用
            damage -= self.get_block();
            *self.block_mut() = 0;
            *self.hp_mut() -= damage;
            println!("{}は{}ダメージを受けた", self.get_name(), damage);
            if  self.get_hp() <= 0 {
                self.set_dead();
                println!("{}は倒れた！", self.get_name());
            }
        }
    }

    // ブロックを獲得する
    fn obtain_block(&mut self, amount: &i32) {
        println!("{}はブロックを{}獲得した", self.get_name(), amount);
        *self.block_mut() += amount;
    }

    fn get_statuses(&self) -> &Statuses;
    fn statuses_mut(&mut self) -> &mut Statuses;
    fn apply_status(&mut self, status_def: &StatusDef, amount: &i32) {
        self.statuses_mut().insert(status_def, *amount);
        if status_def.is_positive {
            println!("{}は{}を{}獲得した", self.get_name(), status_def.name, amount);
        } else {
            println!("{}は{}を{}受けた", self.get_name(), status_def.name, amount);
        }
    }
}