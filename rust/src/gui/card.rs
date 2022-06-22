use common::{
    card::{CardId, CardState, CardType, Unit},
    mana::{Mana, ManaColor, ManaForm},
    player::{PlayerData, PlayerId, PlayerState},
};
use conv::*;
use gdnative::{
    api::{CanvasItem, Control, Label},
    core_types::Color,
    object::{Ref, TRef},
    prelude::*,
};

use crate::{
    resources::Prefabs,
    utils::{vec2, Vec2},
};

pub enum CardView {
    FrontCardView(FrontCardView),
    BackCardView(BackCardView),
}
impl CardView {
    pub fn card_type_change(
        &mut self,
        owner: &Node,
        prefabs: &mut Prefabs,
        state: CardState,
    ) -> Self {
        match self {
            Self::FrontCardView(front) => {
                let card_node = unsafe { front.node.assume_unique() };
                let pos = card_node.global_position();
                card_node.queue_free();
                let back = BackCardView::new(owner, prefabs);
                let card_node = unsafe { back.node.assume_unique() };
                card_node.set_global_position(pos, false);
                Self::BackCardView(back)
            }
            Self::BackCardView(back) => {
                let card_node = unsafe { back.node.assume_unique() };
                let pos = card_node.global_position();
                card_node.queue_free();
                let front = FrontCardView::new(owner, prefabs, state);
                let card_node = unsafe { front.node.assume_unique() };
                card_node.set_global_position(pos, false);
                Self::FrontCardView(front)
            }
        }
    }
    pub fn is_back(&self) -> bool {
        match self {
            &CardView::BackCardView(_) => true,
            &CardView::FrontCardView(_) => false,
        }
    }
    pub fn set_position(&mut self, pos: Vec2) {
        match self {
            CardView::BackCardView(back) => unsafe { back.node.assume_safe() },
            CardView::FrontCardView(front) => unsafe { front.node.assume_safe() },
        }
        .set_global_position(pos, false);
    }
    pub fn hovered_on(&mut self, card_offset: Vec2) -> Vec2 {
        let node = match self {
            CardView::BackCardView(back) => unsafe { back.node.assume_safe() },
            CardView::FrontCardView(front) => unsafe { front.node.assume_safe() },
        };
        let cached_pos = node.global_position();
        // z-index +1
        node.set_scale(vec2(1.5, 1.5));
        node.set_global_position(cached_pos - card_offset, false); //pos - card_offset

        cached_pos
    }
    pub fn hovered_off(&mut self, pos: Vec2) {
        let node = match self {
            CardView::BackCardView(back) => unsafe { back.node.assume_safe() },
            CardView::FrontCardView(front) => unsafe { front.node.assume_safe() },
        };
        // z-index -1
        node.set_scale(vec2(1.0, 1.0));
        node.set_global_position(pos, false); //pos - card_offset
    }
}

pub struct BackCardView {
    node: Ref<Control>,
}
impl BackCardView {
    pub fn new(owner: &Node, prefabs: &mut Prefabs) -> Self {
        let prefab_card = prefabs.card.take().unwrap();
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
        prefabs.card.replace(prefab_obj.claim());
        Self {
            node: card_node.claim(),
        }
    }
}
pub struct FrontCardView {
    node: Ref<Control>,
    name: Ref<Label>,
    // pub labels: [Option<Ref<Label>>; 3],
    mana_cost: Vec<Option<Ref<Label>>>,
    state: CardStateView,
}
impl FrontCardView {
    pub fn new(owner: &Node, prefabs: &mut Prefabs, state: CardState) -> Self {
        let card_node: TRef<Control> = match state.card_type {
            CardType::Unit(_) => {
                let prefab_card_unit = prefabs.card_unit.take().unwrap();
                let prefab_obj = unsafe { prefab_card_unit.assume_safe() };
                let card_unit_node = prefab_obj
                    .instance(0)
                    .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
                    .expect("Could not load player scene");
                // card_unit_node.set_global_position(pos, false);
                owner.add_child(card_unit_node, false);
                //name load json
                //load stats

                prefabs.card_unit.replace(prefab_obj.claim());
                card_unit_node
            }
            _ => {
                let prefab_card_spell = prefabs.card_spell.take().unwrap();
                let prefab_obj = unsafe { prefab_card_spell.assume_safe() };
                let card_spell_node = prefab_obj
                    .instance(0)
                    .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
                    .expect("Could not load player scene");
                // card_spell_node.set_global_position(pos, false);
                // let pos = unsafe { card.get_child(0).unwrap().assume_safe() }
                //     .cast::<TextureRect>()
                //     .unwrap()
                //     .size();
                owner.add_child(card_spell_node, false);
                //name load json
                //load stats
                prefabs.card_spell.replace(prefab_obj.claim());
                card_spell_node
            }
        };
        let CardState {
            name,
            hash,
            cost,
            card_type,
            description,
        } = state.clone();

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
                            let prefab_mana = prefabs.mana.take().unwrap();
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
                            prefabs.mana.replace(prefab_obj.claim());
                            Self::new_mana(mana_node, mana)
                        })
                        .collect()
                })
                .expect("efefefefe"),
            state: card_node
                .get_node("Stats")
                .map(|scene| match card_type {
                    CardType::Unit(unit) => {
                        CardStateView::Unit(UnitView::new(unsafe { scene.assume_safe() }, unit))
                    }
                    // CardType::Spell(spell) => SpellView{}
                    _ => CardStateView::Spell(SpellView {}),
                })
                .expect("Couldn't load sprite texture"),
            node: card_node.claim(),
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

pub enum CardStateView {
    Unit(UnitView),
    Spell(SpellView),
    Build(BuildView),
    Item(ItemView),
    Zone(ZoneView),
}
pub struct UnitView {
    brute_force: Ref<Label>,
    intelligence: Ref<Label>,
    magical_potential: Ref<Label>,
    adaptability: Ref<Label>,
    mastery: Ref<Label>,

    // attack_type: AttackType, DamageType,
    attack: Ref<Label>,
    healty: Ref<Label>,
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
