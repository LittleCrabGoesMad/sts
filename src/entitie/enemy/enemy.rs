// use crate::effect::EffectDef;

use crate::battle::{BattleEntity, BattleScript, StatusDef, Statuses};

pub struct EnemyDef {
    pub name :&'static str,
    pub max_hp :i32,
    pub enemy_script: BattleScript,
}

impl EnemyDef {
    pub fn into_battle(&self, enemy_index: usize) -> Enemy {
        Enemy { 
            name: self.name, 
            battle_entity: BattleEntity::Enemy(enemy_index),
            hp: self.max_hp,
            block: 0,
            statuses: Statuses::new(),
            is_dead: false,
            enemy_script: self.enemy_script,
        }
    }
}
pub struct Enemy {
    pub name :&'static str,
    pub battle_entity: BattleEntity,
    pub hp :i32,
    pub block: i32,
    pub statuses: Statuses,
    pub is_dead :bool,
    pub enemy_script: BattleScript,
}

impl Enemy {
    pub fn take_damage(&mut self, amount: &i32) {
        self.hp -= amount;
        println!("{}は{}ダメージを受けた", self.name, amount);
        if self.hp <= 0 {
            self.is_dead = true;
            println!("{}は倒れた！", self.name);
        }
    }
    
    pub fn obtain_block(&mut self, amount: &i32) {
        self.block += amount;
        println!("{}は{}ブロックを得た", self.name, amount);
    }

    // ステータスを適用する
    pub fn apply_status(&mut self, status_def: &StatusDef, amount: &i32) {
        let statuses :&mut Statuses = &mut self.statuses;
        statuses.insert(status_def, *amount);
        if status_def.is_positive {
            println!("{}は{}を{}獲得した", self.name, status_def.name, amount);
        } else {
            println!("{}は{}を{}受けた", self.name, status_def.name, amount);
        }
    }
}
