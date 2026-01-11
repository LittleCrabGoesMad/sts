use crate::{battle::{Target, battle::BattleContext}, effect::Effect};

pub struct EffectResolver;

impl EffectResolver {
    // 単一のEffectを処理する
    pub fn apply(&self, effect: Effect, context: &mut BattleContext) {
        match effect {
            Effect::DealDamage { amount, target } => {
                match target {
                    Target::BattleAscender => context.battle_ascender.take_damage(&amount),
                    Target::Enemy(index) => context.enemies[index].take_damage(&amount),
                    _ => println!("予期しないエラー"),
                }
            }
            Effect::ObtainBlock { amount, target } => {
                match target {
                    Target::BattleAscender => context.battle_ascender.obtain_block(&amount),
                    Target::Enemy(index) => context.enemies[index].obtain_block(&amount),
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
