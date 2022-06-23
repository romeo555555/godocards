use core::fmt;
use std::slice::Iter;

use bitflags::bitflags;

use crate::*;

pub type PlayerId = u64; //String;

//remove default?
#[derive(Default, Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct PlayerState {
    pub is_controlled: bool,
    tabel: Line,
    hand: Line,
    deck: DeckState,
    factories: FactoriesState,
    equipment: EquipmentState,
    character: CharacterState,
}
impl PlayerState {
    pub const CAPACITY_CARD_ON_TABEL: usize = 8;
    pub const CAPACITY_CARD_ON_HAND: usize = 8;
    pub const CAPACITY_CARD_ON_DECK: usize = 10;
    pub const CAPACITY_CARD_ON_ITEMS: usize = 8;
    pub const CAPACITY_CARD_ON_BUILDS: usize = 8;

    pub fn new(is_controlled: bool) -> Self {
        Self {
            is_controlled,
            ..Default::default()
        }
    }
    pub fn get_tabel(&self) -> &Line {
        &self.tabel
    }
    pub fn get_hand(&self) -> &Line {
        &self.hand
    }
    // pub fn get_all_cards(&self) -> Vec<CardId> {
    //     let mut vec = self.get_tabel();
    //     vec.append(&mut self.get_hand());
    //     vec
    // }
    //del?
    pub fn need_update(&mut self) -> bool {
        // self.flags.player
        true
    }

    // pub fn get_name(&self) -> String {
    //     self.data.name.clone()
    // }
    // pub fn is_client(&self) -> bool {
    //     self.is_client
    // }
    // // pub fn add_card_on_tabel(&mut self, card_id: CardId) {
    // //     self.tabel.add_card(card_id);
    // // }
    pub fn cast_on_tabel(&mut self, card_id: CardId) {
        self.hand.remove_card(card_id);
        self.tabel.add_card(card_id);
    }
    pub fn add_on_hand(&mut self, card_id: CardId) {
        self.hand.add_card(card_id);
    }
    // // pub fn swap_card_on_hand(&mut self, origin_draw_id: CardId, target_card_id: CardId) {
    // //     self.hand.swap_card(origin_draw_id, target_card_id);
    // // }
    // // pub fn back_on_hand(&mut self, hand: &mut Line, tabel: &mut Line, card_id: u64) {
    // //     tabel.remove_card(card_id);
    // //     hand.front_add_card(card_id);
    // // }
    // //remove_card_on_tabel

    // pub fn mana_update(&mut self, count: u64, color: ManaColor) {
    //     self.builds.update(count, color);
    // }
    // // pub fn print_mana_pool(&self) -> String {
    // //     self.mana.print()
    // // }
    // // pub fn try_pay_mana(&mut self, mana_cost: &Vec<ManaForm>) -> bool {
    // //     self.mana.try_pay(mana_cost)
    // // }
    // // pub fn add_mana(&mut self, mana: Mana) {
    // //     self.mana.add_mana(mana);
    // // }

    // pub fn get_card_id(&mut self) -> String {
    //     // self.vec_card
    //     //     .get(rand::gen_range(0usize, 4usize))
    //     //     .unwrap()
    //     //     .clone()
    //     "sss    ".to_owned()
    // }
    // // pub fn update_position(&mut self, res: &mut Resources) {
    // //     self.hand.set_position(res);
    // //     self.tabel.set_position(res);
    // // }
    // //     pub get_gui_state()->(){
    // //         pub struct Player {
    // //             tabel: [i64;4],
    // //             hand: [i64;4],
    // //             items: Items, //equpment
    // //             character: Character,
    // //             deck: Deck,
    // //             builds: ManaPool,
    // //         }
    // //         struct Items {
    // //             count: i64,
    // //             items: Vec<i64>,
    // //         }
    // //         struct Character {
    // //             healty: i64,
    // //         }
    // //         struct Deck {
    // //             card_count: i64,
    // //             dead_deck_count: i64,
    // //             dead_deck: Vec<i64>,
    // //         }
    // //         struct ManaPool {
    // //             count: i64,
    // //             items: Vec<i64>,
    // //         }
    // //    }
    // pub fn sort_line(
    //     &mut self,
    //     ctx: &mut Resources,
    //     line_type: LineType,
    //     exclude_card: Option<CardId>,
    // ) {
    //     let exclude_card = if self.is_client() { exclude_card } else { None };
    //     self.get_line(line_type)
    //         .sort_line(ctx, line_type, exclude_card)
    // }
    // pub fn get_line(&mut self, line_type: LineType) -> &mut Line {
    //     match line_type {
    //         LineType::Hand => &mut self.hand,
    //         LineType::Tabel => &mut self.tabel,
    //     }
    // }
}
#[derive(Copy, Clone, Debug)]
pub enum LineType {
    Tabel,
    Hand,
}
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct Line {
    cards: Vec<CardId>,
    count: f32,
}
impl Line {
    pub fn add_card(&mut self, card_id: CardId) {
        self.count += 1.;
        self.cards.push(card_id);
    }
    pub fn remove_card(&mut self, card_id: CardId) {
        if let Some(idx) = self.cards.iter().position(|&x| x == card_id) {
            self.count -= 1.;
            self.cards.remove(idx);
        }
    }
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    pub fn len_float(&self) -> f32 {
        self.count
    }
    pub fn iter(&self) -> Iter<CardId> {
        self.cards.iter()
    }
    pub fn get_cards(&self) -> &Vec<CardId> {
        &self.cards
    }
    pub fn get(&self, index: usize) -> Option<CardId> {
        self.cards.get(index).cloned()
    }
}
impl Default for Line {
    fn default() -> Self {
        Self {
            cards: Vec::with_capacity(PlayerState::CAPACITY_CARD_ON_HAND),
            count: 0.,
        }
    }
}
#[derive(Default, Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct EquipmentState {
    count: i64,
    items: Vec<i64>,
}
#[derive(Default, Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct DeckState {
    card_count: usize,
    dead_deck_count: i64,
    dead_deck: Vec<i64>,
}
#[derive(Default, Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct CharacterState {}
#[derive(Default, Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct FactoriesState {
    builds: Vec<i64>,
}
//change type to bitflag

// #[derive(Default, Debug, Clone, SerBin, DeBin, PartialEq)]
// pub struct FlagsForUpdate {
//     pub player: bool,
//     pub tabel: bool,
//     pub hand: bool,
//     pub equipment: bool,
//     pub character: bool,
//     pub factories: bool,
//     pub deck: bool,
// }
// impl FlagsForUpdate {
//     pub fn all_true() -> Self {
//         Self {
//             player: true,
//             tabel: true,
//             hand: true,
//             equipment: true,
//             character: true,
//             factories: true,
//             deck: true,
//         }
//     }
//     pub fn all_false() -> Self {
//         Self {
//             player: false,
//             tabel: false,
//             hand: false,
//             equipment: false,
//             character: false,
//             factories: false,
//             deck: false,
//         }
//     }
// }

// #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
// pub struct ClientPlayerData {
//     // pub vec_card: [HashCard; Player::CAPACITY_CARD_ON_DECK],
//     pub data: PlayerData,
// }
//change privats?
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct PlayerData {
    pub name: String,
    pub avatar: String,
    pub deck_name: String,
    pub equipment_name: String,
    pub factories_name: String,
    pub character_name: String,
}

// pub enum PID {
//     Player1,
//     Opp1,
//     Opp2,
//     Friend,
// }
// pub struct Players {
//     player1: Player,
//     player2: Player,
//     player3: Option<Player>,
//     player4: Option<Player>,
// }
// impl Players {
//     pub fn new(client: Player, opp1: Player) -> Self {
//         Self {
//             client,
//             opp1,
//             opp2: None,
//             friend: None,
//         }
//     }
//     pub fn new2x2(client: Player, opp1: Player, opp2: Player, friend: Player) -> Self {
//         Self {
//             client,
//             opp1,
//             opp2: Some(opp2),
//             friend: Some(friend),
//         }
//     }
// }

// impl Index<PID> for Players {
//     type Output = gui::player::Player;

//     fn index(&self, pid: PID) -> &Self::Output {
//         match pid {
//             PID::Client => &self.client,
//             PID::Opp1 => &self.opp1,
//             PID::Opp2 => &self.opp2.unwrap(),
//             PID::Friend => &self.friend.unwrap(),
//         }
//     }
// }
// impl IndexMut<PID> for Players {
//     fn index_mut(&mut self, pid: PID) -> &mut Self::Output {
//         match pid {
//             PID::Client => &mut self.client,
//             PID::Opp1 => &mut self.opp1,
//             PID::Opp2 => &mut self.opp2.unwrap(),
//             PID::Friend => &mut self.friend.unwrap(),
//         }
//     }
// }
