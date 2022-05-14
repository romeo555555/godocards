use crate::*;
use gdnative::api::Node2D;
use gdnative::object::Ref;
use gdnative::prelude::Control;
use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Card {
    pub id: CardId,
    pub node: Ref<Control>,
    pub stats: Option<CardStats>, // is_none == Flipped
}
impl Card {
    // pub fn texture(&self) -> String {
    //     self.stats.hash.clone()
    // }
}

// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub struct CardStats {
//     pub name: String,
//     pub hash: HashCard,
//     pub cost: Vec<ManaForm>,
//     pub card_type: CardType,
//     pub description: String,
// }
// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub struct Unit {
//     brute_force: u64,
//     intelligence: u64,
//     magical_potential: u64,
//     adaptability: u64,
//     mastery: u64,

//     // attack_type: AttackType, DamageType,
//     pub attack: u64,
//     pub healty: u64,
// }
// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub struct Spell {
//     // multiply_damage: u64, //type magic//tochnosty
// }
// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub struct Build {}
// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub struct Item {}
// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub struct Zone {}
// #[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin)]
// pub enum CardType {
//     Unit(Unit),
//     Spell(Spell),
//     Build(Build),
//     Item(Item),
//     Zone(Zone),
// }
// impl Default for CardType {
//     fn default() -> Self {
//         CardType::Flipped
//     }
// }

// #[derive(Clone, Debug, Default)]
// pub struct CardBuilder {
//     render_state: RenderState,
//     stats: CardStats,
// }
// impl CardBuilder {
//     // pub fn ron(mut self) {}
//     pub fn json(mut self, json: &str) -> Self {
//         self.stats = DeJson::deserialize_json(json).unwrap();
//         self
//     }
//     // pub fn build(self, name_id: String) -> Card {
//     //     Card {
//     //         name_id,
//     //         x: 0.,
//     //         y: 0.,
//     //         render_state: self.render_state,

//     //         stats: self.stats,
//     //     }
//     // }
// }
// #[derive(Clone, Copy, Debug, DeJson, SerJson)]
// pub enum RenderState {
//     NoneDraw,
//     Default,
//     Highlight,
//     Hovered,
//     Dragging,
// }

//TODO not crypto hasing fn
// impl Hash for Card {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//     }
// }
// impl PartialEq for Card {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }
// }
// impl Eq for Card {}
