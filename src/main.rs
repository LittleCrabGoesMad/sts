mod effect;
mod battle;
mod card;
mod entitie;

use entitie::{Ascender, EnemyDef};

use crate::battle::{BattleResult, battle_start};
use crate::entitie::enemy::preset::cultist::CULTIST;
use crate::entitie::ascender::preset::ironclad::IRONCLAD;

fn main() {
    let mut ascender: Ascender = IRONCLAD.instantiate();
    let enemys: Vec<&EnemyDef> = vec![&CULTIST];
    let result: BattleResult = battle_start(&ascender, enemys);
    result.apply_result(&mut ascender);
}
