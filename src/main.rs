mod battle;
mod card;
mod entitie;
mod game;

use colored::Colorize;
use entitie::{Ascender, EnemyDef};
use game::{Supply, add_reward_card};

use crate::battle::{BattleResult, battle_start};
use crate::entitie::enemy::preset::acid_slime::ACID_SLIME;
use crate::entitie::enemy::preset::cultist::CULTIST;
use crate::entitie::ascender::preset::ironclad::IRONCLAD;
use crate::entitie::enemy::preset::small_slime::SMALL_SLIME;

fn main() {
    let ascender_def = &IRONCLAD;
    let mut ascender: Ascender = ascender_def.instantiate();
    let mut supply: Supply = Supply::new(ascender_def);
    let enemys: Vec<&EnemyDef> = vec![&CULTIST];
    let result: BattleResult = battle_start(&ascender, enemys);
    if !result.apply_result(&mut ascender){
        println!("{}","ゲームオーバー".red().bold());
        return;
    };
    add_reward_card(&mut ascender, &mut supply);
    let enemys: Vec<&EnemyDef> = vec![&SMALL_SLIME, &ACID_SLIME];
    let result: BattleResult = battle_start(&ascender, enemys);
    if !result.apply_result(&mut ascender){
        println!("{}","ゲームオーバー".red().bold());
        return;
    };
}