// game/reward.rs
// ゲーム中の報酬に関するモジュール
use crate::card::{CardId, get_card_name};
use crate::entitie::Ascender;
use crate::game::Supply;
use crate::get_input_number;

// サプライの通常報酬からカードをマスターデッキに加える
pub fn add_reward_card (ascener: &mut Ascender, supply: &mut Supply) {
    let card_id = offer_reward_cards(supply, 3);
    ascener.add_card_to_master_deck(card_id);
}

// 報酬カードを提示して、１枚選ばせる
fn offer_reward_cards(supply: &mut Supply, amount: usize) -> CardId {
    let mut offered_cards = Vec::new();
    for _ in 0..amount {
        if let Some(card_id) = supply.normal_reward_cards.pop() {
            offered_cards.push(card_id);
        }
    }
    println!("以下の{}枚から1枚を選んでください:", amount);
    for (index, card_id) in offered_cards.iter().enumerate() {
        println!("{}: {:?}", index + 1, get_card_name(*card_id));
    }
    let mut choice: usize;
    loop {
        choice = get_input_number();
        if choice == 0 || choice > offered_cards.len() {
            println!("1から{}の間の数字を入力してください", offered_cards.len());
        } else {
            break
        }
    }
    offered_cards[choice - 1]
}