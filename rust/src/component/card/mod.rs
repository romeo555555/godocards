use bevy_ecs::prelude::*;
use common::{CardId, CardStats, Mana, ManaColor, ManaForm, PlayerId};
use conv::*;
use gdnative::{
    api::{CanvasItem, Control, Label},
    core_types::Color,
    object::{Ref, TRef},
    prelude::*,
};

use crate::{
    resources::Resources,
    utils::{vec2, Rect},
};

#[derive(Component)]
pub struct OnTabel {}
#[derive(Component)]
pub struct OnHand {}

#[derive(Bundle)]
pub struct CardBundle {
    card: Card,
    card_ui: CardUI,
}
impl CardBundle {
    pub fn new(owner: &Node, ctx: &mut Resources, id: CardId) -> Self {
        Self {
            card: Card { id, stats: None },
            card_ui: CardUI::new(owner, ctx),
        }
    }
}
#[derive(Component)]
pub struct Card {
    id: CardId,
    stats: Option<CardStats>, // is_none == Flipped
}
#[derive(Component)]
pub struct CardUI {
    node: Ref<Control>,
}
impl CardUI {
    pub fn new(owner: &Node, ctx: &mut Resources) -> Self {
        if let Some(prefab_card) = ctx.prefab_card.take() {
            let prefab_obj = unsafe { prefab_card.assume_safe() };
            let card_node = prefab_obj
                .instance(0)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
                .expect("Could not load player scene");
            // card.set_position(vec2(150., 180.) * vec2(hash_id as f32, hash_id as f32));
            // let pos = unsafe { card.get_child(0).unwrap().assume_safe() }
            //     .cast::<TextureRect>()
            //     .unwrap()
            //     .size();
            owner.add_child(card_node, false);
            //name load json
            //load stats
            ctx.prefab_card.replace(prefab_obj.claim());
            CardUI {
                node: card_node.claim(),
            }
        } else {
            panic!("Not found prefab_card")
        }
    }
}
#[derive(Component)]
pub struct CardStatsUI {
    name: Ref<Label>,
    // pub labels: [Option<Ref<Label>>; 3],
    mana_cost: Vec<Option<Ref<Label>>>,
}
impl CardStatsUI {
    fn new(card_node: TRef<Control>, ctx: &mut Resources, stats: CardStats) -> Self {
        let CardStats {
            name,
            hash,
            cost,
            card_type,
            description,
        } = stats.clone();

        Self {
            name: card_node
                .get_node("Name")
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Label>())
                .map(|scene| {
                    scene.set_text(name);
                    scene
                })
                .expect("Couldn't load sprite texture")
                .claim(),
            mana_cost: card_node
                .get_node("Cost")
                .map(|scene| unsafe { scene.assume_safe() })
                .map(|scene| {
                    cost.into_iter()
                        .enumerate()
                        .map(|(i, mana)| {
                            let prefab_mana = ctx.prefab_mana.take().unwrap();
                            let prefab_obj = unsafe { prefab_mana.assume_safe() };
                            let mana_node = prefab_obj
                                .instance(0)
                                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
                                .expect("Could not load player scene");

                            scene.add_child(mana_node, false);
                            mana_node.set_position(
                                vec2(150. - (35. * f32::value_from(i + 1).unwrap()), 0.),
                                false,
                            );
                            ctx.prefab_mana.replace(prefab_obj.claim());
                            Self::new_mana(mana_node, mana)
                        })
                        .collect()
                })
                .expect("efefefefe"),
            // stats: card_node
            //     .get_node("Stats")
            //     .map(|scene| match card_type {
            //         CardType::Unit(unit) => {
            //             CardTypeView::Unit(UnitView::new(unsafe { scene.assume_safe() }, unit))
            //         }
            //         // CardType::Spell(spell) => SpellView{}
            //         _ => CardTypeView::Spell(SpellView {}),
            //     })
            //     .expect("Couldn't load sprite texture"),
        }
    }

    pub fn new_mana(mana_node: TRef<Control>, mana: Mana) -> Option<Ref<Label>> {
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

        Some(
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

#[derive(Component)]
pub enum CardTypeUI {
    Unit(UnitUI),
    Spell(SpellUI),
    Build(BuildUI),
    Item(ItemUI),
    Zone(ZoneUI),
}
#[derive(Component)]
pub struct UnitUI {
    brute_force: Ref<Label>,
    intelligence: Ref<Label>,
    magical_potential: Ref<Label>,
    adaptability: Ref<Label>,
    mastery: Ref<Label>,

    // attack_type: AttackType, DamageType,
    attack: Ref<Label>,
    healty: Ref<Label>,
}
#[derive(Component)]
pub struct SpellUI {
    // multiply_damage: u64, //type magic//tochnosty
}
#[derive(Component)]
pub struct BuildUI {}
#[derive(Component)]
pub struct ItemUI {}
#[derive(Component)]
pub struct ZoneUI {}
