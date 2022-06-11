use bevy_ecs::prelude::*;
pub use card::*;
use common::{CardId, PlayerId};
use gdnative::{
    api::{Control, Label},
    object::Ref,
};
mod card;
use crate::*;
use gdnative::{api::TextureRect, prelude::*};
use std::collections::HashMap;

use crate::utils::Rect;

#[derive(Component)]
pub struct PlayerUI {
    player_id: PlayerId,
    //player_type
}
#[derive(Component)]
pub struct PlayerRect {
    rect: Rect,
}
#[derive(Bundle)]
pub struct PlayerBundle {
    // tabel: Vec<CardId>,
    // hand: Vec<CardId>,
    player_rect: PlayerRect,
    tabel_rect: TabelRect,
    hand_rect: HandRect,
    equipment_rect: EquipmentRect,
    character_rect: CharacterRect,
    deck_rect: DeckRect,
    factories_rect: FactoriesRect,

    tabel_ui: TabelUI,
    hand_ui: HandUI,
    equipment_ui: EquipmentUI,
    character_ui: CharacterUI,
    deck_ui: DeckUI,
    factories_ui: FactoriesUI,
}
impl PlayerBundle {
    pub fn new(node: Option<Ref<Node>>) -> Self {
        let (tabel_ui, tabel_rect) = (
            TabelUI {},
            TabelRect {
                rect: Rect::default(),
            },
        );
        let (hand_ui, hand_rect) = (
            HandUI {},
            HandRect {
                rect: Rect::default(),
            },
        );
        let (equipment_ui, equipment_rect) = EquipmentUI::new(node, "items".to_owned());
        let (character_ui, character_rect) = CharacterUI::new(node, "avatarmini1".to_owned());
        let (deck_ui, deck_rect) = DeckUI::new(node, "deck".to_owned());
        let (factories_ui, factories_rect) = FactoriesUI::new(node, "builds".to_owned());

        Self {
            player_rect: PlayerRect {
                rect: Rect::default(),
            },
            tabel_rect,
            tabel_ui,
            hand_ui,
            hand_rect,
            equipment_ui,
            equipment_rect,
            character_ui,
            character_rect,
            deck_ui,
            deck_rect,
            factories_ui,
            factories_rect,
        }
    }
}
#[derive(Component)]
pub struct TabelUI {}
#[derive(Component)]
pub struct TabelRect {
    rect: Rect,
}
#[derive(Component)]
pub struct HandUI {}
#[derive(Component)]
pub struct HandRect {
    rect: Rect,
}

#[derive(Component)]
pub struct EquipmentRect {
    rect: Rect,
}
#[derive(Component)]
pub struct EquipmentUI {
    label_name: Ref<Label>,
    label_count: Ref<Label>,
}
impl EquipmentUI {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> (Self, EquipmentRect) {
        let (items, rect) = player_component(player, "Items".to_owned(), texture);
        let label_name = items
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_count = items
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        // godot_print!("Items create: {}", rect);
        (
            Self {
                label_name,
                label_count,
            },
            EquipmentRect { rect },
        )
    }
}
#[derive(Component)]
pub struct DeckRect {
    rect: Rect,
}
#[derive(Component)]
pub struct DeckUI {
    label_card_deck: Ref<Label>,
    label_dead_deck: Ref<Label>,
}
impl DeckUI {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> (Self, DeckRect) {
        let (deck, rect) = player_component(player, "Deck".to_owned(), texture);
        let label_card_deck = deck
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_dead_deck = deck
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        // godot_print!("Deck create: {}", rect);
        (
            Self {
                label_card_deck,
                label_dead_deck,
            },
            DeckRect { rect },
        )
    }
}
#[derive(Component)]
pub struct CharacterRect {
    rect: Rect,
}
#[derive(Component)]
pub struct CharacterUI {
    label_name: Ref<Label>,
    label_healty: Ref<Label>,
}
impl CharacterUI {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> (Self, CharacterRect) {
        let (char, rect) = player_component(player, "Character".to_owned(), texture);
        let label_name = char
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_healty = char
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        // godot_print!("Character create: {}", rect);
        (
            Self {
                label_name,
                label_healty,
            },
            CharacterRect { rect },
        )
    }
}
#[derive(Component)]
pub struct FactoriesRect {
    rect: Rect,
}

#[derive(Component)]
pub struct FactoriesUI {
    label_red: Ref<Label>,
    label_blue: Ref<Label>,
    label_green: Ref<Label>,
    label_white: Ref<Label>,
    label_black: Ref<Label>,
}
impl FactoriesUI {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> (Self, FactoriesRect) {
        let (builds, rect) = player_component(player, "Builds".to_owned(), texture);
        (
            Self {
                label_red: builds
                    .get_node("Red")
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim(),
                label_blue: builds
                    .get_node("Red")
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim(),
                label_green: builds
                    .get_node("Red")
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim(),
                label_white: builds
                    .get_node("Red")
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim(),
                label_black: builds
                    .get_node("Red")
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim(),
            },
            FactoriesRect { rect },
        )
        // godot_print!("Builds create: {}", rect);
    }
    // pub fn update(&mut self, count: u64, color: ManaColor) {
    //     if let Some(label) = self.labels.get_mut(match color {
    //         ManaColor::Red => 0,
    //         ManaColor::Blue => 1,
    //         ManaColor::Green => 2,
    //         ManaColor::White => 3,
    //         ManaColor::Black => 4,
    //     }) {
    //         unsafe { label.assume_safe() }.set_text(count.to_string());
    //     }
    //     // .unwrap();
    // }
}
fn player_component<'a>(
    player: Option<Ref<Node, Shared>>,
    name: String,
    texture: String,
) -> (TRef<'a, TextureRect>, Rect) {
    let scene = player
        .and_then(|scene| unsafe { scene.assume_safe() }.get_node(name))
        .and_then(|scene| unsafe { scene.assume_safe().cast::<TextureRect>() })
        .map(|scene| {
            scene.set_texture(
                ResourceLoader::godot_singleton()
                    .load(
                        format!("res://assets/sprites/{}.png", texture),
                        "Texture",
                        false,
                    )
                    .and_then(|res| res.cast::<Texture>())
                    .expect("Couldn't load sprite texture"),
            );
            scene
        })
        .expect("Couldn't load sprite texture");
    let pos = scene.global_position();
    let size = scene.size();
    (scene, Rect::new(pos.x, pos.y, size.x, size.y))
}
