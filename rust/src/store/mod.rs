use crate::{
    gui::Gui,
    input_action::{PlayerType, SelectAction},
    network::{self, Network, ServerApi},
    selecting::{SelectedState, SelectingCard},
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
    server_api: ServerApi,
    client_id: PlayerId,
    bd_cards: HashMap<HashCard, CardState>,
}
impl Store {
    pub fn new(
        players_state: HashMap<PlayerId, PlayerState>,
        cards_state: HashMap<CardId, Option<CardState>>,
        server_api: ServerApi,
        client_id: PlayerId,
        bd_cards: HashMap<HashCard, CardState>,
    ) -> Self {
        Self {
            players_state,
            cards_state,
            server_api,
            client_id,
            bd_cards,
        }
    }
    // pub fn dispatch(
    //     &mut self,
    //     match_action: MatchAction,
    //     res: &mut Resources,
    //     selected_card: &mut SelectingCard,
    // ) {
    //     match match_action {
    //         MatchAction::Server(msg) => {
    //             let ServerMessage { player_id, action } = msg;
    //             match action {
    //                 ServerAction::TakeCard(card_id) => {
    //                     self.players_state
    //                         .get_mut(&player_id)
    //                         .unwrap()
    //                         .push_hand(card_id);
    //                     self.players_flags
    //                         .get_mut(&player_id)
    //                         .unwrap()
    //                         .insert(FlagsForUpdate::HAND);
    //                 }
    //                 ServerAction::CastCardOnTabel(card_id) => {
    //                     selected_card.drop();
    //                     self.players_state
    //                         .get_mut(&player_id)
    //                         .unwrap()
    //                         .push_tabel(card_id);
    //                     self.players_flags
    //                         .get_mut(&player_id)
    //                         .unwrap()
    //                         .insert(FlagsForUpdate::TABEL);
    //                 }
    //                 ServerAction::ManaUpdate(count, mana_color) => {
    //                     //     self.gui.get_player(&player_id).mana_update(count, color);
    //                 }
    //                 ServerAction::FlipCard(card_id, hash_card) => {
    //                     *self.cards_state.get_mut(&card_id).unwrap() =
    //                         Some(res.get_card_from_bd(hash_card));
    //                 }
    //                 ServerAction::RemoveCard(card_id) => {}
    //                 ServerAction::BackCardOnHand(card_id) => {}
    //                 ServerAction::ChangeState(match_state) => {}
    //             }
    //         }
    //         MatchAction::Input(input) => match input {
    //             InputAction::Hovered(input_type, player_type) => {
    //                 //hovered
    //                 // if let InputType::TabelCard(card_id) | InputType::HandCard(card_id) = input_type
    //                 // {
    //                 //     selected_card.hovered(card_id);
    //                 // }
    //             }
    //             InputAction::Clicked(input_type, player_type) => {
    //                 if let Some(action) = cliked(self.client_flags(), input_type, selected_card) {
    //                     self.server_api.send(action);
    //                 };
    //             }
    //             InputAction::Dragging(input_type, player_type) => {
    //                 //only exclude type
    //                 // if let Some(action) = dragging(self.client_flags(), input_type, selected_card) {
    //                 //                         self.server_api.send(action);
    //                 //                     };
    //             }
    //             InputAction::Drop(input_type, player_type) => {
    //                 if let Some(action) = drop(self.client_flags(), input_type, selected_card) {
    //                     self.server_api.send(action);
    //                 };
    //             }
    //         },
    //     }
    // }
    // pub fn update_gui_state(
    //     &mut self,
    //     gui: &mut Gui,
    //     res: &mut Resources,
    //     exclude_card: Option<CardId>,
    // ) {
    //     let iter = self.players_flags.iter();
    //     iter.for_each(|(id, flags)| {
    //         if !flags.is_empty() {
    //             let state = self.players_state.get(id).unwrap();
    //             gui.update(
    //                 id,
    //                 state,
    //                 flags,
    //                 res,
    //                 if state.is_controlled {
    //                     exclude_card
    //                 } else {
    //                     None
    //                 },
    //             );
    //         }
    //     });
    pub fn get_players_state_map(&self) -> &HashMap<PlayerId, PlayerState> {
        &self.players_state
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
}

pub fn dispatch(
    action: Action,
    owner: &Node,
    store: &mut Store,
    gui: &mut Gui,
    network: &mut Network,
    selecting_card: &mut SelectingCard,
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
                    gui.sort_hand(&player_id, state.get_hand());
                }
                ServerAction::CastCardOnTabel(card_id) => {
                    selecting_card.drop();
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
                    gui.sort_tabel(&player_id, state.get_tabel());
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
                    // let card_state = res.get_card_from_bd(hash_card);
                    // store
                    //     .cards_state
                    //     //if is_some dont insert
                    //     .insert(card_id, Some(card_state.clone()));
                    // gui.flip_card(&card_id, owner, res, card_state);
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
                selecting_card.drag(card_id);
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
            SelectAction::Hover(card_id) => selecting_card.hovered(card_id),
            SelectAction::Drop => {
                selecting_card.drop();
                let player_id = network.client_id;
                let state = store.get_mut_player_state(&player_id);
                gui.sort_hand(&player_id, state.get_hand());
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
