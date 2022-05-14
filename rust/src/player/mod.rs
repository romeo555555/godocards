use crate::*;
use gdnative::api::Texture;
use gdnative::object::Ref;
use std::{cmp::Ordering, ops::Add};
pub mod components;
use components::*;
use nanoserde::{DeBin, SerBin};

pub struct Player {
    pub(crate) player_id: PlayerId,
    pub(crate) rect: Rect,
    pub(crate) tabel: Tabel,
    pub(crate) hand: Hand,
    pub(crate) deck: Deck,
    pub(crate) items: Items,
    pub(crate) builds: Builds,
    pub(crate) character: Character,
    //avatar
    pub(crate) healty: u64,
    pub(crate) data: PlayerData,
}
impl Player {
    pub const CAPACITY_CARD_ON_HAND: usize = 8;
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

    pub fn mana_update(&mut self, mana: Mana) {
        self.builds.update(mana);
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
    pub fn new(rect: Rect, label_name: RefLabel, label_count: RefLabel) -> Self {
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
    pub fn new(rect: Rect, label_name: RefLabel, label_healty: RefLabel) -> Self {
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
    pub fn new(rect: Rect, label_card_deck: RefLabel, label_dead_deck: RefLabel) -> Self {
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
    pub fn new(rect: Rect, labels: Vec<RefLabel>) -> Self {
        Self { rect, labels }
    }
    pub fn update(&mut self, mana: Mana) {
        let (idx, count) = match mana {
            Mana::Red(count) => (0, count),
            Mana::Blue(count) => (1, count),
            Mana::Green(count) => (2, count),
            Mana::White(count) => (3, count),
            Mana::Black(count) => (4, count),
        };
        self.labels
            .get_mut(idx)
            .map(|label| {
                unsafe { label.assume_safe() }.set_text(count.to_string());
            })
            .unwrap();
    }
}
