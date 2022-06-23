use common::{
    card::{CardId, CardState},
    game_match::MatchInfo,
    mana::{Mana, ManaColor, ManaForm},
    player::{Line, PlayerData, PlayerId, PlayerState},
};
mod card;
use card::*;
use gdnative::{
    api::{CanvasItem, Control, Label},
    core_types::Color,
    object::{Ref, TRef},
    prelude::*,
};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    slice::{Iter, SliceIndex},
};

use crate::utils::{vec2, Rect};

use crate::*;
use gdnative::api::{Node, ResourceLoader, TextureRect};
use gdnative::prelude::Shared;
use gdnative::{api::Texture, prelude::godot_print};
use nanoserde::{DeBin, SerBin};
use std::collections::VecDeque;
use std::{cmp::Ordering, ops::Add};

pub struct Gui {
    prefabs: Prefabs,
    players_view: HashMap<PlayerId, PlayerView>,
    cards_view: HashMap<CardId, CardView>,
}
impl Gui {
    pub fn new(
        owner: &Node,
        match_info: MatchInfo,
        // client_id: PlayerId,
    ) -> (Self, Store, Layout) {
        let mut prefabs = Prefabs::init(); //TODO:Up on ierarhi and in Option<Prefabs>
        let match_scene = ResourceLoader::godot_singleton()
            .load("res://Match.tscn", "PackedScene", false)
            .and_then(|res| {
                let res = unsafe { res.assume_thread_local() };
                res.cast::<PackedScene>()
            })
            .and_then(|packed_scene| packed_scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED))
            .and_then(|scene| {
                let scene = unsafe { scene.assume_safe() };
                scene.cast::<Node2D>()
            })
            .expect("Could not load player scene");
        owner.add_child(match_scene, false);

        let MatchInfo {
            client_id,
            players_state,
            players_data,
            cards,
            bd_cards,
        } = match_info;

        //TODO:new_1x1 return LAYOUT.2x2
        let layout = Layout::new(MatchType::Match1x1);
        let players_view: HashMap<PlayerId, PlayerView> = players_data
            .into_iter()
            .map(|(id, player_data)| {
                if id == client_id {
                    (
                        id,
                        PlayerView::new(
                            match_scene.get_child(1),
                            player_data,
                            &layout.client,
                            PlayerType::Client,
                            true,
                        ),
                    )
                } else {
                    (
                        id,
                        PlayerView::new(
                            match_scene.get_child(0),
                            player_data,
                            &layout.opp1,
                            PlayerType::Opp1,
                            false,
                        ),
                    )
                }
            })
            .collect();

        let cards_view = cards
            .iter()
            .map(|(id, state)| {
                let view = if let Some(state) = state {
                    CardView::FrontCardView(FrontCardView::new(owner, &mut prefabs, state.clone()))
                } else {
                    CardView::BackCardView(BackCardView::new(owner, &mut prefabs))
                };
                (*id, view)
            })
            .collect();
        let mut gui = Self {
            prefabs,
            players_view,
            cards_view,
        };
        players_state.iter().for_each(|(id, state)| {
            let layout = if *id == client_id {
                &layout.client
            } else {
                &layout.opp1
            };
            layout.sort_tabel(state, &mut gui);
            layout.sort_hand(state, &mut gui);
        });
        (
            gui,
            Store::new(players_state, cards, client_id, bd_cards),
            layout,
        )
    }
    pub fn create_card(&mut self, card_id: CardId, owner: &Node) {
        self.cards_view.insert(
            card_id,
            CardView::BackCardView(BackCardView::new(owner, &mut self.prefabs)),
        );
    }
    pub fn flip_card(&mut self, card_id: &CardId, owner: &Node, card_state: CardState) {
        let card_view = self
            .cards_view
            .get_mut(card_id)
            .expect("ERROR: Not foud card");
        if card_view.is_back() {
            let new_card_view = card_view.card_type_change(owner, &mut self.prefabs, card_state);
            self.cards_view.insert(*card_id, new_card_view);
        }
    }
    pub fn get_mut_card(&mut self, card_id: &CardId) -> &mut CardView {
        self.cards_view
            .get_mut(card_id)
            .expect("ERROR: Not foud card")
    }
    pub fn get_player_type(&self, player_id: &PlayerId) -> PlayerType {
        self.players_view.get(player_id).unwrap().player_type()
    }
}

pub struct PlayerView {
    player_type: PlayerType,
    tabel: LineView,
    hand: LineView,
    deck: DeckView,
    factories: FactoriesView,
    equipment: EquipmentView,
    character: CharacterView,
    //avatar
}
impl PlayerView {
    pub const CAPACITY_CARD_ON_TABEL: usize = 8;
    pub const CAPACITY_CARD_ON_HAND: usize = 8;
    pub const CAPACITY_CARD_ON_DECK: usize = 8;
    pub const CAPACITY_CARD_ON_ITEMS: usize = 8;
    pub const CAPACITY_CARD_ON_BUILDS: usize = 8;
    pub fn new(
        player: Option<Ref<Node>>,
        player_data: PlayerData,
        layout: &LayoutPlayer,
        player_type: PlayerType,
        is_client: bool,
    ) -> Self {
        PlayerView {
            player_type,
            tabel: LineView::new(),
            hand: LineView::new(),
            deck: DeckView::new(player, layout.deck, player_data.deck_name),
            factories: FactoriesView::new(player, layout.factories, player_data.factories_name),
            equipment: EquipmentView::new(player, layout.equipment, player_data.equipment_name),
            character: CharacterView::new(player, layout.character, player_data.character_name),
            // avatar:
            // data: player_data.data,
            // is_client,
            // player_id: player_data.id.clone(),
        }
    }
    pub fn player_type(&self) -> PlayerType {
        self.player_type
    }
}
pub struct LineView {
    // rect: Rect,
}
impl LineView {
    pub fn new() -> Self {
        Self {}
    }
}
pub struct EquipmentView {
    label_name: Ref<Label>,
    label_count: Ref<Label>,
}
impl EquipmentView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let items = player_component(player, "Items".to_owned(), rect, texture);
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
        godot_print!("Items create: {}", rect);
        Self {
            label_name,
            label_count,
        }
    }
    pub fn update(&mut self) {}
}
pub struct CharacterView {
    label_name: Ref<Label>,
    label_healty: Ref<Label>,
}
impl CharacterView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let char = player_component(player, "Character".to_owned(), rect, texture);
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
        godot_print!("Character create: {}", rect);
        Self {
            label_name,
            label_healty,
        }
    }
    pub fn update(&mut self) {}
}
pub struct DeckView {
    label_card_deck: Ref<Label>,
    label_dead_deck: Ref<Label>,
}

impl DeckView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let deck = player_component(player, "Deck".to_owned(), rect, texture);
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
        godot_print!("Deck create: {}", rect);
        Self {
            label_card_deck,
            label_dead_deck,
        }
    }
    pub fn update(&mut self) {}
}
pub struct FactoriesView {
    label_red: Ref<Label>,
    label_blue: Ref<Label>,
    label_green: Ref<Label>,
    label_white: Ref<Label>,
    label_black: Ref<Label>,
}
impl FactoriesView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let builds = player_component(player, "Builds".to_owned(), rect, texture);
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
        }
        // godot_print!("Builds create: {}", rect);
    }
    pub fn update(&mut self) {}
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
    rect: Rect,
    texture: String,
) -> TRef<'a, TextureRect> {
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
    scene.set_global_position(rect.point(), false);
    scene.set_size(rect.size(), false);
    scene
}
// fn avatar( player: Option<Ref<Node>>, texture: String, pos: Vec2) -> Deck {
//     let deck = player_component(player, "Avatar", texture);
//     deck.set_position(pos, false);
//     let label_card_count = deck
//         .get_child(0)
//         .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
//         .expect("Couldn't load sprite texture")
//         .claim();
//     let label_dead_count = deck
//         .get_child(0)
//         .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
//         .expect("Couldn't load sprite texture")
//         .claim();
//     let size = deck.size();
//     Deck::new(
//         Rect::new(pos.x, pos.x, size.x, size.y),
//         label_card_count,
//         label_dead_count,
//     )
// }
