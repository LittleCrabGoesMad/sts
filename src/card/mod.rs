// card/mod.rs
mod battle_deck;
mod card_def;
mod card_instance;

pub use battle_deck::BattleDeck;
pub use card_def::{CardDef, CardId, CardType, get_card_def, get_card_name};
pub use card_instance::CardInstance;