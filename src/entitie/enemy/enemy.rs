// use crate::effect::EffectDef;

use crate::battle::BattleScript;

pub struct EnemyDef {
    pub name :&'static str,
    pub max_hp :i32,
    pub enemy_script: BattleScript,
}

impl EnemyDef {
    pub fn into_battle(&self) -> Enemy {
        Enemy { 
            name: self.name, 
            hp: self.max_hp,
            block: 0,
            is_dead: false,
            enemy_script: self.enemy_script,
        }
    }
}
pub struct Enemy {
    pub name :&'static str,
    pub hp :i32,
    pub block: i32,
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
}
