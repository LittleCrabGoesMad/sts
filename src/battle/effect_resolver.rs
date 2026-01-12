use crate::battle::{BattleContext, BattleEntity, Effect, STRENGTH, VULNERABLE};

pub struct EffectResolver;

impl EffectResolver {
    // 単一のEffectを処理する
    pub fn apply(&self, source: BattleEntity, effect: Effect, context: &mut BattleContext) {
        match effect {
            // ダメージを与える
            Effect::DealDamage { amount, target } => {
                let mut damage = amount;
                match source {
                    BattleEntity::BattleAscender => damage += context.battle_ascender.statuses.get(&STRENGTH),
                    BattleEntity::Enemy(index) => damage += context.enemies[index].statuses.get(&STRENGTH),
                    _ => println!("予期しないエラー"),
                }
                match target {
                    BattleEntity::BattleAscender => {
                        if context.battle_ascender.statuses.can_consume(&VULNERABLE, 1) {
                            damage = damage * 2;
                        }
                        context.battle_ascender.take_damage(&damage);
                    }
                    BattleEntity::Enemy(index) => {
                        if context.enemies[index].statuses.can_consume(&VULNERABLE, 1) {
                            damage = damage * 2;
                        }
                        context.enemies[index].take_damage(&damage);
                    }
                    _ => println!("予期しないエラー"),
                }
            }

            // ブロックを獲得する
            Effect::ObtainBlock { amount, target } => {
                match target {
                    BattleEntity::BattleAscender => context.battle_ascender.obtain_block(&amount),
                    BattleEntity::Enemy(index) => context.enemies[index].obtain_block(&amount),
                    _ => println!("予期しないエラー"),
                }
            }

            // ステータスを適用する
            Effect::ApplyStatus { status_def, amount, target } => {
                match target {
                    BattleEntity::BattleAscender => context.battle_ascender.apply_status(&status_def, &amount),
                    BattleEntity::Enemy(index) => context.enemies[index].apply_status(&status_def, &amount),
                    _ => println!("予期しないエラー"),
                }
            }
        }
        // プレイヤーが死ぬか的が全滅したら戦闘終了フラグを立てる
        if context.battle_ascender.is_dead || context.enemies.iter().all(|enemy| enemy.is_dead) {
            context.is_end_battle = true;
        }
    }
}
