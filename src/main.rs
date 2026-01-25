use std::io::{self, Write};
mod battle;
mod card;
mod entitie;
mod game;

use colored::Colorize;
use entitie::{Ascender, EnemyDef};
use game::{Supply, add_reward_card};

use crate::battle::{BattleResult, battle_start};
use crate::entitie::AscenderDef;
use crate::entitie::ascender::preset::{IRONCLAD, SILENT};
use crate::entitie::enemy::preset::acid_slime::ACID_SLIME;
use crate::entitie::enemy::preset::cultist::CULTIST;
use crate::entitie::enemy::preset::small_slime::SMALL_SLIME;

fn main() {
    let ascender_def = &choose_ascender();
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

fn choose_ascender() -> AscenderDef {
    let ascender_def: AscenderDef;
    loop {
        // アイアンクラッドかサイレントを選択する処理
        println!("アセンダーを選択してください:");
        println!("1. アイアンクラッド");
        println!("2. サイレント");
        let choice = get_input_number();
        if choice == 1 {
            ascender_def = IRONCLAD;
            break;
        } else if choice == 2 {
            ascender_def = SILENT;
            break;
        } else {
            println!("1か2を入力してください");
        }
   }
   ascender_def
}
    
pub fn get_input_number() -> usize {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        // 入力行をうかせるための改行
        println!();

        // 数字以外は弾く
        let Ok(num) = trimmed.parse::<usize>() else {
            println!("半角数字を入力してください");
            continue;
        };

        return num;
    }
}