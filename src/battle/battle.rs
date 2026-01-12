
use crate::battle::{BattleSelector, BattleEntity};
use crate::battle::battle_view::BattleView;
use crate::battle::effect_resolver::EffectResolver;
use crate::entitie::{Ascender, BattleAscender, Enemy, EnemyDef};

// 戦闘の結果
pub struct BattleResult {
    pub hp_after: i32,
    pub victory: bool,
}

impl BattleResult {
    pub fn apply_result(self, ascender: &mut Ascender) {
        if self.victory {
            println!("戦闘に勝利！");
            ascender.current_hp = self.hp_after;
        } else {
            println!("死...");
        }
    }
}

// 戦闘のコンテキスト
pub struct BattleContext {
    pub battle_ascender: BattleAscender,
    pub enemies: Vec<Enemy>,
    turn: u32,
    pub is_end_battle: bool,
    pub battle_selector: BattleSelector,
}

impl BattleContext {
    pub fn new(ascender: &Ascender, enemies_def: Vec<&EnemyDef>) -> Self {
        let battle_ascender = ascender.into_battle();
        // 敵定義から戦闘用の敵を生成,敵のインデックスを付与
        let enemies: Vec<Enemy> = enemies_def.into_iter().enumerate().map(|(index, enemy)| enemy.into_battle(index)).collect();
        BattleContext {
            battle_ascender,
            enemies,
            turn: 0,
            is_end_battle: false,
            battle_selector: BattleSelector,
        }
    }

    // 戦闘のビューを生成する
    pub fn create_view(&self) -> BattleView {
        BattleView {
            ascender: &self.battle_ascender,
            enemies: &self.enemies,
        }
    }

    // ターン開始処理
    fn start_ascender_turn (&mut self) {
        self.turn += 1;
        println!("-- ターン {} 開始 --", self.turn);
        self.battle_ascender.start_turn();
    }

    // ターン終了処理
    fn end_ascender_turn (&mut self) {
        self.battle_ascender.end_turn();
        println!("-- ターン {} 終了 --", self.turn);
    }

    // カードを使う
    fn try_play_card (&mut self) -> Option<BattleScript> {
        // 使うカードをプレイヤーの入力で決定する
        loop {
            // ここで0を入力するとターンが終了する
            let target_card = self.battle_selector.choose_card_for_play(&self.create_view())?;
            let chosen_card_index = match target_card {
                BattleEntity::Card(index) => index,
                _ => panic!("Invalid target for card index"),
            };
            if !self.battle_ascender.can_use_card(chosen_card_index) {
                println!("エナジーが足りない......");
                continue;
            }
            let card_script: BattleScript = self.battle_ascender.play_card(chosen_card_index);
            return Some(card_script);
        }
    }
}

// 各エンティティが実行する戦闘スクリプトの型
// ここに展開された時点で、主体から切り離されているため、第一引数として主体を記述する
pub type BattleScript = fn(&BattleEntity, &EffectResolver, &mut BattleContext);

pub fn battle_start(ascender: &Ascender, enemies_def: Vec<&EnemyDef>) -> BattleResult {
    let mut context: BattleContext = BattleContext::new(&ascender, enemies_def);
    let resolver: EffectResolver = EffectResolver;
    println!("=== 戦闘開始 ===");

    loop {
        // アセンダーのターン
        context.start_ascender_turn();
        while !context.is_end_battle {
            // カード使用一回分の処理
            if let Some(card_script) = context.try_play_card() {
                card_script(&BattleEntity::BattleAscender, &resolver, &mut context);
            } else {
                // ターン終了処理
                break;
            }
        }
        if context.is_end_battle {
            break;
        }
        context.end_ascender_turn();

        // 敵のターン
        //　借用の都合上、敵の行動を一体ずつ処理する
        let enemy_count = context.enemies.len();
        for i in 0..enemy_count {
            let enemy: &Enemy = &context.enemies[i];
            if enemy.is_dead {
                continue;
            }
            // 敵の行動スクリプトを実行
            let enemy_entity = enemy.battle_entity;
            let enemy_sctipt = enemy.enemy_script;
            enemy_sctipt(&enemy_entity, &resolver, &mut context);
            if context.is_end_battle {
                break;
            }
        }
        if context.is_end_battle {
            break;
        }
    }

    println!("=== 戦闘終了 ===");
    context.battle_ascender.out_of_battle()
}
