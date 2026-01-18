use std::thread::sleep;
use std::time::Duration;

use crate::battle::battle::CombatantId;
use crate::battle::{BattleContext, Effect, STRENGTH, VULNERABLE};
use crate::entitie::Combatant;

pub struct EffectResolver;

impl EffectResolver {
    // 単一のEffectを処理する
    pub fn apply(&self, source: CombatantId, effect: Effect, context: &mut BattleContext) {
        if context.is_end_battle {
            // 戦闘が終了している場合は何もしない
            return;
        }
        let source_combatant = context.combatant(source);
        match effect {
            // ダメージを与える
            Effect::DealDamage { amount, target } => {
                let mut damage = amount;
                let strength = source_combatant.get_statuses().get(&STRENGTH);
                    damage += strength;
                let target_combatant = context.combatant_mut(target);
                if target_combatant.statuses_mut().can_consume(&VULNERABLE, 1) {
                    damage = damage * 2;
                }
                target_combatant.take_damage(&damage);
            }

            // ブロックを獲得する
            Effect::ObtainBlock { amount, target } => {
                let target_combatant = context.combatant_mut(target);
                target_combatant.obtain_block(&amount);
            }

            // ステータスを適用する
            Effect::ApplyStatus { status_def, amount, target } => {
                let target_combatant = context.combatant_mut(target);
                target_combatant.apply_status(&status_def, &amount);
            }

            // カードを引く
            Effect::DrawCards { amount, target } => {
                if target == CombatantId::Ascender {
                    context.battle_ascender.draw_cards(amount as usize);
                }
            }

        }
        sleep(Duration::from_millis(750)); // 効果適用の間に少し待機

        // 死んだ敵を戦闘から除外する
        context.enemies.retain(|enemy| !enemy.is_dead());

        // プレイヤーが死ぬか敵が全滅したら戦闘終了フラグを立てる
        if context.battle_ascender.is_dead() || context.enemies.iter().all(|enemy| enemy.is_dead()) {
            context.is_end_battle = true;
        }
    }
}
