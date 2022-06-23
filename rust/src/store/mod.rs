use crate::{
    gui::Gui,
    input::{Input, PlayerType, SelectAction},
    layout::Layout,
    network::{self, Network, ServerApi},
};
use core::fmt;
use std::collections::{hash_map::IterMut, HashMap};
// mod state;
use bitflags::bitflags;
use common::{
    card::{CardId, CardState, HashCard},
    player::{PlayerId, PlayerState},
    ClientAction, ClientMessage, ServerAction, ServerMessage,
};
use gdnative::api::Node;
#[derive(Clone, Debug)]
pub enum Action {
    Server(ServerMessage),
    Client(ClientAction),
    Select(SelectAction),
}

pub struct Store {
    players_state: HashMap<PlayerId, PlayerState>,
    cards_state: HashMap<CardId, Option<CardState>>,
    client_id: PlayerId,
    bd_cards: HashMap<HashCard, CardState>,
}
impl Store {
    pub fn new(
        players_state: HashMap<PlayerId, PlayerState>,
        cards_state: HashMap<CardId, Option<CardState>>,
        client_id: PlayerId,
        bd_cards: HashMap<HashCard, CardState>,
    ) -> Self {
        Self {
            players_state,
            cards_state,
            client_id,
            bd_cards,
        }
    }
    pub fn get_players_state_map(&self) -> &HashMap<PlayerId, PlayerState> {
        &self.players_state
    }
    pub fn get_player_state_from_type(&self, player_type: &PlayerType) -> &PlayerState {
        match player_type {
            &PlayerType::Client => self.players_state.get(&self.client_id).unwrap(),
            _ => self
                .players_state
                .iter()
                .find(|(id, state)| *id != &self.client_id)
                .map(|(_, state)| state)
                .unwrap(),
        }
    }
    pub fn get_player_state(&self, player_id: &PlayerId) -> &PlayerState {
        self.players_state.get(player_id).unwrap()
    }
    pub fn get_mut_player_state(&mut self, player_id: &PlayerId) -> &mut PlayerState {
        self.players_state.get_mut(player_id).unwrap()
    }
    // pub fn iter_mut_player(&mut self) -> IterMut<PlayerId, PlayerState> {
    //     self.players_state.iter_mut()
    // }
    pub fn get_card_from_bd(&self, card_hash: HashCard) -> CardState {
        self.bd_cards.get(&card_hash).unwrap().clone()
    }
}

pub fn dispatch(
    action: Action,
    owner: &Node,
    input: &mut Input,
    store: &mut Store,
    gui: &mut Gui,
    network: &mut Network,
    layout: &Layout,
) {
    match action {
        Action::Server(msg) => {
            let ServerMessage { player_id, action } = msg;
            match action {
                ServerAction::TakeCard(card_id) => {
                    let state = store.get_mut_player_state(&player_id);
                    state.add_on_hand(card_id);
                    gui.create_card(card_id, owner);
                    // if player_id == client_id
                    //if take card    and dragging -> card drop
                    // if let Some(exclude_card) = selecting_card.get_id_if_dragging() {
                    //     gui.sort_hand_witch_exclude(
                    //         &player_id,
                    //         state.get_hand(),
                    //         res.card_size(),
                    //         res.card_indent(),
                    //         exclude_card,
                    //     );
                    // } else {
                    if player_id == network.client_id {
                        &layout.client
                    } else {
                        &layout.opp1
                    }
                    .sort_hand(state, gui);
                }
                ServerAction::CastCardOnTabel(card_id) => {
                    input.drop();
                    let state = store.get_mut_player_state(&player_id);
                    state.cast_on_tabel(card_id);
                    // self.players_state
                    //     .get_mut(&player_id)
                    //     .unwrap()
                    //     .push_tabel(card_id);
                    match gui.get_player_type(&player_id) {
                        PlayerType::Client => {}
                        PlayerType::Opp1 => {}
                        PlayerType::Opp2 => {}
                        PlayerType::Friendly => {}
                    }
                    // gui.sort_hand(
                    //     &player_id,
                    //     state.get_hand(),
                    //     res.card_size(),
                    //     res.card_indent(),
                    // );
                    let layout = if player_id == network.client_id {
                        &layout.client
                    } else {
                        &layout.opp1
                    };
                    layout.sort_tabel(state, gui);
                    layout.sort_hand(state, gui);
                    // self.players_state
                    //     .get_mut(&player_id)
                    //     .unwrap()
                    //     .push_tabel(card_id);
                    // self.players_flags
                    //     .get_mut(&player_id)
                    //     .unwrap()
                    //     .insert(FlagsForUpdate::TABEL);
                }
                ServerAction::ManaUpdate(count, mana_color) => {
                    //     self.gui.get_player(&player_id).mana_update(count, color);
                }
                ServerAction::FlipCard(card_id, hash_card) => {
                    let card_state = store.get_card_from_bd(hash_card);
                    store
                        .cards_state
                        //if is_some dont insert
                        .insert(card_id, Some(card_state.clone()));
                    gui.flip_card(&card_id, owner, card_state);
                }
                ServerAction::RemoveCard(card_id) => {}
                ServerAction::BackCardOnHand(card_id) => {}
                ServerAction::ChangeState(match_state) => {}
            }
        }
        Action::Client(action) => network.send_msg(action),
        //         match action {
        //     ClientAction::TakeCard => network.send_msg(action),
        //     _ => {}
        // },
        Action::Select(selecting) => match selecting {
            SelectAction::Drag(card_id) => {
                input.drag(card_id);
                // let player_id = network.client_id;
                // let state = store.get_mut_player_state(&player_id);
                // gui.sort_hand_witch_exclude(
                //     &player_id,
                //     state.get_hand(),
                //     res.card_size(),
                //     res.card_indent(),
                //     card_id, //exclude_card,
                // );
            }
            SelectAction::Hover(card_id) => input.hovered(card_id),
            SelectAction::Drop => {
                input.drop();
                // let player_id = network.client_id;
                // let state = store.get_mut_player_state(&player_id);
                // if player_id == network.client_id {
                //     &layout.client
                // } else {
                //     &layout.opp1
                // }
                let state = store.get_mut_player_state(&network.client_id);
                &layout.client.sort_hand(state, gui);
            }
        },
    }
}
// bitflags! {
//     #[derive(Default)]
//     pub struct FlagsForUpdate : u32 {
//         const TABEL      = 0b00000001;
//         const HAND       = 0b00000010;
//         const EQUIPMENT  = 0b00000100;
//         const CHARACTER  = 0b00001000;
//         const FACTORIES  = 0b00010000;
//         const DECK       = 0b00100000;
//         const ALL     = Self::TABEL.bits
//                            | Self::HAND.bits
//                            | Self::EQUIPMENT.bits
//                            | Self::CHARACTER.bits
//                            | Self::FACTORIES.bits
//                            | Self::DECK.bits;
//     }
// }
// impl FlagsForUpdate {
//     pub fn clear(&mut self) -> &mut Self {
//         self.bits = 0;
//         self
//     }
// }
// impl fmt::Display for FlagsForUpdate {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:032b}", self.bits)
//     }
// }
