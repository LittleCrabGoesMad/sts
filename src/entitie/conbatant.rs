// entitie/conbatant.rs
// 戦闘に参加する存在のトレイトを定義するモジュール

use crate::battle::{STRENGTH, StatusDef, Statuses, VULNERABLE, WEAK};

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

    // 自身のステータスに基づいて与えるダメージを修正する
    fn modify_outgoing_damage(&mut self, base_damage: i32, needs_consume: bool) -> i32 {
        let mut modified_damage = base_damage;

        // 筋力によるダメージ増加
        if let Some(strength) = self.get_statuses().get_stacks(&STRENGTH) {
            modified_damage += strength;
        }

        // 脱力によるダメージ減少
        if self.get_statuses().is_exist(&WEAK) {
            modified_damage -= 1;
            if needs_consume {
                self.statuses_mut().can_consume(&WEAK, 1);
            }
        }

        // ダメージは0未満にならないようにする
        if modified_damage < 0 {
            modified_damage = 0;
        }

        modified_damage
    }

    // 自身のステータスに基づいて受けるダメージを修正する
    // 単なるダメージ確認用の場合はneeds_consumeをfalseにする
    fn modify_incoming_damage(&mut self, base_damage: i32, needs_consume: bool) -> i32 {
        let mut modified_damage = base_damage;

        // 弱体化によるダメージ増加
        // 弱体化があるか確認し、ある場合はダメージを二倍にする
        if self.get_statuses().is_exist(&VULNERABLE) {
            modified_damage = modified_damage * 2;
            if needs_consume {
                self.statuses_mut().can_consume(&VULNERABLE, 1);
            }
        }

        // ダメージは0未満にならないようにする
        if modified_damage < 0 {
            modified_damage = 0;
        }
        modified_damage
    }
}