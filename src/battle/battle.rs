use crate::battle::BattleSelector;
use crate::battle::battle_view::BattleView;
use crate::battle::effect_resolver::EffectResolver;
use crate::entitie::{Ascender, BattleAscender, Combatant, Enemy, EnemyDef};

// 戦闘の結果
pub struct BattleResult {
    pub hp_after: i32,
    pub victory: bool,
}

impl BattleResult {
    pub fn apply_result(self, ascender: &mut Ascender) -> bool{
        if self.victory {
            println!("戦闘に勝利！");
            ascender.current_hp = self.hp_after;
        } else {
            println!("死...");
        }
        self.victory
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
        let enemies: Vec<Enemy> = enemies_def.iter().map(|def| def.into_battle()).collect();
        BattleContext {
            battle_ascender,
            enemies,
            turn: 0,
            is_end_battle: false,
            battle_selector: BattleSelector,
        }
    }

    // 指定されたCombatantIdに対応するCombatantへの不変参照を返す
    #[allow(dead_code)]
    pub fn combatant(&self, id: CombatantId) -> &dyn Combatant {
        match id {
            CombatantId::Ascender => &self.battle_ascender,
            CombatantId::Enemy(index) => &self.enemies[index],
        }
    }

    // 指定されたCombatantIdに対応するCombatantへの可変参照を返す
    pub fn combatant_mut(&mut self, id: CombatantId) -> &mut dyn Combatant {
        match id {
            CombatantId::Ascender => &mut self.battle_ascender,
            CombatantId::Enemy(index) => &mut self.enemies[index],
        }
    }   

    // 戦闘のビューを生成する
    pub fn create_view<'view>(&'view self) -> BattleView<'view> {
        BattleView {
            ascender: &self.battle_ascender,
            enemies: &self.enemies,
        }
    }

    // ターン開始処理
    fn start_ascender_turn (&mut self) {
        self.turn += 1;
        self.battle_ascender.start_turn();
    }

    // ターン終了処理
    fn end_ascender_turn (&mut self) {
        self.battle_ascender.end_turn();
    }

    // カードを使う
    fn try_play_card (&mut self) -> Option<BattleScript> {
        // 使うカードをプレイヤーの入力で決定する
        loop {
            // ここで0を入力するとターンが終了する
            let chosen_card_index = self.battle_selector.choose_card_for_play(self.create_view())?;
            if !self.battle_ascender.can_use_card(chosen_card_index) {
                self.create_view().render_not_enough_energy();
                continue;
            }
            let card_script: BattleScript = self.battle_ascender.play_card(chosen_card_index);
            return Some(card_script);
        }
    }
}

// 各エンティティが実行する戦闘スクリプトの型
// ここに展開された時点で、主体から切り離されているため、第一引数として主体を記述する
pub type BattleScript = fn(CombatantId, &EffectResolver, &mut BattleContext);

// スクリプトを実行する主体を識別するための列挙型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CombatantId {
    Ascender,
    Enemy(usize),
}

pub fn battle_start(ascender: &Ascender, enemies_def: Vec<&EnemyDef>) -> BattleResult {
    let mut context: BattleContext = BattleContext::new(&ascender, enemies_def);
    let resolver: EffectResolver = EffectResolver;
    
    context.create_view().render_battle_start();
    loop {
        // アセンダーのターン
        context.start_ascender_turn();
        context.create_view().render_turn_start();
        while !context.is_end_battle {
            // カード使用一回分の処理
            context.create_view().render_combatant_summary();
            if let Some(card_script) = context.try_play_card() {
                card_script(CombatantId::Ascender, &resolver, &mut context);
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
            if enemy.is_dead() {
                continue;
            }
            // 敵の行動スクリプトを実行
            let enemy_sctipt = enemy.enemy_script;
            enemy_sctipt(CombatantId::Enemy(i), &resolver, &mut context);
            if context.is_end_battle {
                break;
            }
        }
        if context.is_end_battle {
            break;
        }
    }
    context.battle_ascender.out_of_battle()
}
