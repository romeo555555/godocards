use crate::*;
use gdnative::api::{Label, Node, ResourceLoader, TextureRect};
use gdnative::object::{Ref, TRef};
use gdnative::prelude::Shared;
use gdnative::{api::Texture, prelude::godot_print};
use std::{cmp::Ordering, ops::Add};
pub mod components;
use components::*;
use nanoserde::{DeBin, SerBin};

pub struct Player {
    player_id: PlayerId,
    rect: Rect,
    tabel: Tabel,
    hand: Hand,
    deck: Deck,
    items: Items,
    builds: Builds,
    character: Character,
    //avatar
    healty: u64,
    data: PlayerData,
}
impl Player {
    pub const CAPACITY_CARD_ON_HAND: usize = 8;
    pub fn new(
        player: Option<Ref<Node>>,
        rect: Rect,
        player_data: PlayerDataHandler,
        card_size: Vec2,
        tabel_rect: Rect,
        hand_rect: Rect,
    ) -> Player {
        Player {
            player_id: player_data.id.clone(),
            rect,
            healty: 100,
            tabel: Tabel::new(tabel_rect, Player::CAPACITY_CARD_ON_HAND, card_size),
            hand: Hand::new(hand_rect, Player::CAPACITY_CARD_ON_HAND, card_size),
            deck: Deck::new(player, player_data.deck_name),
            items: Items::new(player, player_data.items_name),
            builds: Builds::new(player, player_data.builds_name),
            character: Character::new(player, player_data.character_name),
            // avatar:
            data: player_data.data,
        }
    }

    pub fn get_name(&self) -> String {
        self.data.name.clone()
    }
    // pub fn player_type(&self) -> PlayerType {
    //     self.player_type
    // }
    pub fn contains(&self, sense: Sense) -> bool {
        sense.contains_rect(&self.rect)
    }
    pub fn input_handler(&self, sense: Sense) -> Response {
        let click_up = sense.click_up;
        let click_down = sense.click_down;
        Response::new(
            self.components_contains(sense),
            self.player_id,
            click_up,
            click_down,
        )
    }
    fn components_contains(&self, sense: Sense) -> ResponseType {
        // if match self.player_type {
        //     PlayerType::Client => sense.mouse_x > self.rect.center_x,
        //     // PlayerType::Remote => sense.mouse_x < self.rect.center_x,
        //     PlayerType::Remote => sense.mouse_x > self.rect.center_x,
        // }
        if sense.mouse_x > self.rect.center_x {
            if sense.contains_rect(&self.items.rect) {
                return ResponseType::Items;
            } else if sense.contains_rect(&self.character.rect) {
                return ResponseType::Character;
            }
        } else if sense.contains_rect(&self.deck.rect) {
            return ResponseType::Deck;
        } else if sense.contains_rect(&self.builds.rect) {
            return ResponseType::Builds;
        }
        if self.hand.contains(&sense) {
            return self.hand.input_handler(sense);
        } else if self.tabel.contains(&sense) {
            return self.tabel.input_handler(sense);
        }
        ResponseType::None
    }
    // pub fn add_card_on_tabel(&mut self, card_id: CardId) {
    //     self.tabel.add_card(card_id);
    // }
    // pub fn cast_on_tabel(&mut self, hand: &mut Line, tabel: &mut Line, card_id: u64) {
    //     hand.remove_card(card_id);
    //     tabel.front_add_card(card_id);
    // }
    //remove_card_on_tabel
    pub fn add_card_on_hand(&mut self, card_id: CardId) {
        // hash_card:Option<HashCard>
        self.hand.add_card(card_id);
    }
    // pub fn swap_card_on_hand(&mut self, origin_draw_id: CardId, target_card_id: CardId) {
    //     self.hand.swap_card(origin_draw_id, target_card_id);
    // }
    // pub fn back_on_hand(&mut self, hand: &mut Line, tabel: &mut Line, card_id: u64) {
    //     tabel.remove_card(card_id);
    //     hand.front_add_card(card_id);
    // }
    //remove_card_on_tabel

    pub fn mana_update(&mut self, count: u64, color: ManaColor) {
        self.builds.update(count, color);
    }
    // pub fn print_mana_pool(&self) -> String {
    //     self.mana.print()
    // }
    // pub fn try_pay_mana(&mut self, mana_cost: &Vec<ManaForm>) -> bool {
    //     self.mana.try_pay(mana_cost)
    // }
    // pub fn add_mana(&mut self, mana: Mana) {
    //     self.mana.add_mana(mana);
    // }

    pub fn get_card_id(&mut self) -> String {
        // self.vec_card
        //     .get(rand::gen_range(0usize, 4usize))
        //     .unwrap()
        //     .clone()
        "sss    ".to_owned()
    }
    pub fn update_position(&mut self, res: &mut Resources) {
        self.hand.set_position(res);
        self.tabel.set_position(res);
    }
}
// pub struct DublePlayer {
// player_id: PlayerId,
//     rect: Rect,
//     general_tabel: Tabel, //not all  unit controlled
//     hand_client: Hand,
//     hand_freind: Hand,
//     deck: Deck,
//     items: Items,
//     builds: Builds,
//     character: Character,
//     //avatar
//     healty: u64,
//     data: PlayerData,
//     player_type: PlayerType,
//     // pub first_chance_order_step: u64,
// }
// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, DeBin, SerBin)]
// pub enum PlayerType {
//     Client,
//     Remote,
// }

// pub struct DoubleSide<P: Player> {}

//component
pub struct Items {
    rect: Rect,
    label_name: RefLabel,
    label_count: RefLabel,
    count: i64,
    items: Vec<i64>,
}
impl Items {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
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
        godot_print!("Items create: {}", rect);
        Self {
            rect,
            label_name,
            label_count,
            count: 0,
            items: Vec::with_capacity(8),
        }
    }
}
pub struct Character {
    rect: Rect,
    label_name: RefLabel,
    label_healty: RefLabel,
}
impl Character {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
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
        godot_print!("Character create: {}", rect);
        Self {
            rect,
            label_name,
            label_healty,
        }
    }
}
pub struct Deck {
    rect: Rect,
    label_card_deck: RefLabel,
    label_dead_deck: RefLabel,
    card_count: i64,
    dead_deck_count: i64,
    dead_deck: Vec<i64>,
}

impl Deck {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
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
        godot_print!("Deck create: {}", rect);
        Self {
            rect,
            label_card_deck,
            label_dead_deck,
            card_count: 30,
            dead_deck_count: 0,
            dead_deck: Vec::with_capacity(8),
        }
    }
}
pub struct Builds {
    rect: Rect,
    labels: Vec<RefLabel>,
}
impl Builds {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
        let (builds, rect) = player_component(player, "Builds".to_owned(), texture);
        //builds
        // 0 - red
        // 1 - blue
        // 2 - green
        // 3 - white
        // 4 - black
        let labels = vec!["Red", "Blue", "Green", "White", "Black"]
            .into_iter()
            .map(|name| {
                builds
                    .get_node(name)
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim()
            })
            .collect();
        godot_print!("Builds create: {}", rect);

        Self { rect, labels }
    }
    pub fn update(&mut self, count: u64, color: ManaColor) {
        self.labels
            .get_mut(match color {
                ManaColor::Red => 0,
                ManaColor::Blue => 1,
                ManaColor::Green => 2,
                ManaColor::White => 3,
                ManaColor::Black => 4,
            })
            .map(|label| {
                unsafe { label.assume_safe() }.set_text(count.to_string());
            });
        // .unwrap();
    }
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
