use crate::*;
use gdnative::api::{CanvasItem, Label, Node, Node2D, TextureRect};
use gdnative::core_types::Color;
use gdnative::object::{Ref, TRef};
use gdnative::prelude::Control;
use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Card {
    // pub id: CardId,
    pub node: Ref<Control>,
    pub stats: Option<CardStats>, // is_none == Flipped
}
impl Card {
    // pub fn texture(&self) -> String {
    //     self.stats.hash.clone()
    // }
}

pub struct ManaView(RefLabel);
impl ManaView {
    pub fn new(mana_node: TRef<Control>, mana: Mana) -> Self {
        let Mana { count, mana_form } = mana;
        match mana_form {
            ManaForm::Once(color) => Self::match_mana(mana_node, 1, color),
            ManaForm::Two(colors) => [1, 2]
                .iter()
                .zip(colors)
                .for_each(|(idx, color)| Self::match_mana(mana_node, *idx, color)),
            ManaForm::Three(colors) => [1, 2, 3]
                .iter()
                .zip(colors)
                .for_each(|(idx, color)| Self::match_mana(mana_node, *idx, color)),
            ManaForm::Four(colors) => [1, 2, 4, 5]
                .iter()
                .zip(colors)
                .for_each(|(idx, color)| Self::match_mana(mana_node, *idx, color)),
            ManaForm::Uncolor => {
                if let Some(node) = mana_node
                    .get_child(6)
                    .and_then(|scene| unsafe { scene.assume_safe() }.cast::<CanvasItem>())
                {
                    node.set_visible(true);
                }
            }
        }

        ManaView(
            mana_node
                .get_child(0)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(count.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),
        )
    }
    fn match_mana(scene: TRef<Control>, idx: i64, color: ManaColor) {
        if let Some(node) = scene
            .get_child(idx)
            .and_then(|scene| unsafe { scene.assume_safe() }.cast::<CanvasItem>())
        {
            node.set_visible(true);
            node.set_modulate(match color {
                ManaColor::Red => Color::from_rgb(255., 0., 0.),
                ManaColor::Blue => Color::from_rgb(0., 255., 0.),
                ManaColor::Green => Color::from_rgb(0., 0., 255.),
                ManaColor::White => Color::from_rgb(0., 0., 0.),
                ManaColor::Black => Color::from_rgb(255., 255., 255.),
            });
        }
    }
}
pub enum CardTypeView {
    Unit(UnitView),
    Spell(SpellView),
    Build(BuildView),
    Item(ItemView),
    Zone(ZoneView),
}
impl CardTypeView {
    pub fn update(card_type_view: CardTypeView) {
        match card_type_view {
            CardTypeView::Unit(view) => {}
            CardTypeView::Spell(view) => {}
            _ => {}
        }
    }
}
pub struct UnitView {
    brute_force: RefLabel,
    intelligence: RefLabel,
    magical_potential: RefLabel,
    adaptability: RefLabel,
    mastery: RefLabel,

    // attack_type: AttackType, DamageType,
    attack: RefLabel,
    healty: RefLabel,
}
impl UnitView {
    pub fn new(stats_node: TRef<Node>, unit: Unit) -> Self {
        Self {
            brute_force: stats_node
                .get_child(0)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.brute_force.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),
            intelligence: stats_node
                .get_child(1)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.intelligence.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),

            magical_potential: stats_node
                .get_child(2)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.magical_potential.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),

            adaptability: stats_node
                .get_child(3)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.adaptability.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),

            mastery: stats_node
                .get_child(4)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.mastery.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),

            attack: stats_node
                .get_child(5)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.attack.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),

            healty: stats_node
                .get_child(6)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(unit.healty.to_string());
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),
        }
    }
}

pub struct SpellView {
    // multiply_damage: u64, //type magic//tochnosty
}
pub struct BuildView {}
pub struct ItemView {}
pub struct ZoneView {}

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
