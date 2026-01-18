// 戦闘における必要なUI表示、または処理に必要な情報を提供するモジュール

use colored::Colorize;

use crate::{battle::Statuses, entitie::{BattleAscender, Combatant, Enemy}};

pub struct BattleView<'view> {
    pub ascender: &'view BattleAscender,
    pub enemies: &'view[Enemy],
}

impl<'view> BattleView<'view> {
    // アセンダーと敵の情報をまとめて表示する
    pub fn render_combatant_summary(&self) {
        // アセンダーの情報を表示
        self.render_split_line();
        self.render_ascender_summary();
        self.render_split_line();

        // 敵の情報を表示
        self.render_enemies_summary();
        self.render_split_line();
        self.render_empty_line();
    }

    // 戦闘開始時の表示
    pub fn render_battle_start(&self) {
        println!("{}", "=== 戦闘開始 ===".green().bold());
    }

    // ターン開始時の表示
    pub fn render_turn_start (&self) {
        println!("--- ターン開始 ---");
        // self.render_combatant_summary();
    }

    // アセンダーの情報を表示する
    fn render_ascender_summary(&self) {
        let ascender = self.ascender;
        println!("{}", ascender.get_name());
        println!("エナジー: {}/{}", ascender.get_current_energy(), ascender.get_max_energy());
        println!("HP: {} | ブロック: {}", ascender.get_hp(), ascender.get_block());
        self.render_statuses(ascender.get_statuses());
    }

    // 敵の情報を表示する
    pub fn render_enemies_summary(&self) {
        for (index, enemy) in self.enemies.iter().enumerate() {
            // インデックスも表示する
            print!("{}: ", index + 1);
            println!("{}", enemy.get_name());
            println!("HP: {} | ブロック: {}", enemy.get_hp(), enemy.get_block());
            self.render_statuses(enemy.get_statuses());
            println!("行動: {}", enemy.action_description);
        }
    }

    // ステータスを一覧表示する
    fn render_statuses(&self, statuses: &Statuses) {
        print!("ステータス: (");
        let mut first_print = true;
        for (status_def, amount) in statuses.list_statuses() {
            if !first_print {
                print!(" | ");
            }
            print!("{}: {} ", status_def.name, amount);
            first_print = false;
        }
        println!(")");
    }

    // 手札の情報を表示する
    pub fn render_hand(&self) {
        for (i, card) in self.ascender.deck.hand.iter().enumerate() {
            let name = self.pad_to_width(card.name, 16);
            let description = self.pad_to_width(card.brief_description, 32);
            println!("{:<2}: {} ({:<1}) {}",
            i + 1,
            name,
            card.cost,
            description);
        }
    }

    // エナジーが足りない場合の表示
    pub fn render_not_enough_energy(&self) {
        println!("{}", "エナジーが足りない......".red().bold());
    }

    // 区切り線を表示する
    fn render_split_line(&self) {
        println!("==========================================");
    }

    // 空行を表示する
    fn render_empty_line(&self) {
        println!();
    }

    // unicode幅に合わせてパディングする
    fn pad_to_width(&self, s: &str, width: usize) -> String {
        let w = unicode_width::UnicodeWidthStr::width(s);
        if w >= width {
            s.to_string()
        } else {
            format!("{}{}", s, " ".repeat(width - w))
        }
    }
}