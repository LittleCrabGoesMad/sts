// use crate::effect::EffectDef;

use crate::{battle::{BattleScript, Statuses}, entitie::Combatant};

pub struct EnemyDef {
    pub name :&'static str,
    pub max_hp :i32,
    pub enemy_script: BattleScript,
    pub action_description: &'static str,
}

impl EnemyDef {
    pub fn into_battle(&self) -> Enemy {
        Enemy { 
            name: self.name, 
            hp: self.max_hp,
            block: 0,
            statuses: Statuses::new(),
            is_dead: false,
            enemy_script: self.enemy_script,
            action_description: self.action_description,
        }
    }
}
pub struct Enemy {
    name :&'static str,
    hp :i32,
    block: i32,
    statuses: Statuses,
    is_dead :bool,
    pub enemy_script: BattleScript,
    pub action_description: &'static str,
}

impl Combatant for Enemy {
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