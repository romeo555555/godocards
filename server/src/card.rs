use crate::*;
use nanoserde::{DeJson, SerJson};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default)] //Default
pub struct Card {
    pub id: CardId,
    // pub node: Ref<Control>,
    pub stats: Option<CardStats>, // is_none == Flipped
}
impl Card {
    // pub fn texture(&self) -> String {
    //     self.stats.hash.clone()
    // }
}
