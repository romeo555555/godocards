use common::card::{CardId, CardState, HashCard};
use common::card_builder::CardStateBuilder;
use common::game_match::MatchInfo;
use common::player::{LineType, PlayerData, PlayerId, PlayerState};
use mana::*;
mod mana;
mod network;
use common::*;
use network::*;
use rand::*;
extern crate common;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self, NodeEvent, NodeHandler, NodeListener};
use nanoserde::{DeBin, DeBinErr, SerBin};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::result;
use std::slice::SliceIndex;
use std::time::Duration;

// #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
enum Signal {
    // MatchFound,
// MatchCreat
}
pub fn main() {
    GameMatch::default().run(
        Transport::FramedTcp,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3042),
    );
}

struct GameMatch {
    // config: Config,
    bd_cards: HashMap<HashCard, CardState>,
    network: Network,
    // state: MatchState,
    players_state: HashMap<PlayerId, PlayerState>,
    players_data: HashMap<PlayerId, PlayerData>,
    cards: HashMap<CardId, CardState>,
    is_ready: bool,
}
impl Default for GameMatch {
    fn default() -> Self {
        Self {
            network: Network::new(),
            bd_cards: CardStateBuilder::new_pool().into_iter().collect(),
            // state: State::None,
            players_state: HashMap::with_capacity(2),
            players_data: HashMap::with_capacity(2),
            cards: HashMap::with_capacity(40),
            is_ready: false,
        }
    }
}
impl GameMatch {
    const NEEDED_PLAYER_FOR_START: usize = 2;
    const HC: [&'static str; 10] = [
        "unit1", "unit2", "unit3", "unit4", "unit5", "unit6", "unit7", "unit7", "wizard1",
        "wizard2",
    ];
    pub fn run(&mut self, transport: Transport, addr: SocketAddr) {
        self.network
            .take_listener(transport, addr)
            .for_each(move |event| match event {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Connected(_, _) => (), // Only generated at connect() calls.
                    NetEvent::Accepted(endpoint, _listener_id) => {
                        // Only connection oriented protocols will generate this event
                        self.network.add_sub(endpoint);
                    }
                    NetEvent::Message(endpoint, data) => {
                        if self.is_ready {
                            if let Result::<ClientMessage, DeBinErr>::Ok(msg) =
                                DeBin::deserialize_bin(data)
                            {
                                println!("take event");
                                self.match_msg(endpoint, msg);
                            }
                        //TODO: } else if let Result::<(PlayerData, [HashCard; 30], DeBinErr>::Ok(player) =
                        } else if let Result::<PlayerData, DeBinErr>::Ok(player) =
                            DeBin::deserialize_bin(data)
                        {
                            self.add_player(endpoint, player);
                        }
                    }
                    NetEvent::Disconnected(endpoint) => {
                        // self.subscriptions.remove(&endpoint);
                        println!(
                            "Client ({}) disconnected (total clients: {})",
                            endpoint.addr(),
                            // self.subscriptions.len()
                            "bye".to_owned()
                        );
                    }
                },
                NodeEvent::Signal(signal) => match signal {
                // let subscriptions: Vec<Endpoint> = self.subscriptions.iter().cloned().collect();
            },
            });
    }
    fn match_msg(&mut self, endpoint: Endpoint, msg: ClientMessage) {
        match msg.action {
            ClientAction::TakeCard => {
                // let player = self.players_state.get_mut(&msg.player_id).unwrap();
                // let hash_card = player.get_random_card_hash();
                // player.add_card_hand(card_id);
                // self.add_card(&hash_card);
                let player_id = self.network.get_sub_id(endpoint);
                let (card_id, hash_card) = self.add_random_card(&player_id);

                self.network.send_msg_for_all(&ServerMessage::build(
                    player_id,
                    ServerAction::TakeCard(card_id),
                ));
                self.network.send_msg(
                    endpoint,
                    &ServerMessage::build(player_id, ServerAction::FlipCard(card_id, hash_card)),
                );
            }
            ClientAction::FlipCard(card_id, hash_card) => {}
            ClientAction::CastCardOnTabel(card_id) => {
                let card = self.cards.get(&card_id).unwrap();
                //card_cost
                let hash_card = card.hash.clone();
                self.network.send_msg_for_all(&ServerMessage::build(
                    msg.player_id,
                    ServerAction::CastCardOnTabel(card_id),
                ));
                self.network.send_msg_for_all(&ServerMessage::build(
                    msg.player_id,
                    ServerAction::FlipCard(card_id, hash_card),
                ));
            }
            ClientAction::BackCardOnHand(card_id) => {}
            ClientAction::EndStep => {}
            _ => {}
        }
    }
    fn add_player(&mut self, endpoint: Endpoint, player_data: PlayerData) {
        let player_id = self.network.get_sub_id(endpoint);
        self.players_state
            .insert(player_id, PlayerState::new(false));
        self.players_data.insert(player_id, player_data);

        if self.players_state.len() == GameMatch::NEEDED_PLAYER_FOR_START {
            self.create_match();
        }
    }

    fn create_match(&mut self) {
        // let mut init_card: HashMap<PlayerId, Vec<(CardId, HashCard)>> =
        //     HashMap::with_capacity(self.players.len());

        // for (player_id, player) in self.players.iter_mut() {
        //     let mut vec = Vec::with_capacity(3);
        //     for _ in 0..3 {
        //         let hash_card = player.get_random_card_hash();
        //         let card_id = self.network.get_card_id();

        //         player.add_card_hand(card_id);
        //         vec.push((card_id, hash_card));
        //     }
        //     init_card.insert(*player_id, vec);
        // }
        // init_card.iter().for_each(|(_, vec)| {
        //     vec.iter().for_each(|(card_id, hash_card)| {
        //         self.cards.insert(
        //             *card_id,
        //             Card {
        //                 id: *card_id,
        //                 stats: self.get_stats_from_bd(hash_card),
        //             },
        //         );
        //     })
        // });
        let keys: Vec<PlayerId> = self
            .players_state
            .iter()
            .map(|(id, _)| id.clone())
            .collect();
        keys.into_iter().for_each(|id| {
            for _ in 0..3 {
                self.add_random_card(&id);
            }
        });
        for (player_id, player) in &self.players_state {
            // let mut start_hand = init_card.clone();
            let mut cards: HashMap<CardId, Option<CardState>> = self
                .cards
                .clone()
                .into_iter()
                .map(|(id, state)| (id, Some(state)))
                .collect();
            let players_state = self
                .players_state
                .clone()
                .into_iter()
                .map(|(id, mut state)| {
                    if *player_id == id {
                        state.is_controlled = true;
                    } else {
                        state.get_hand().iter().for_each(|card_id| {
                            cards.get_mut(card_id).unwrap().take();
                        });
                    }
                    (id, state)
                })
                .collect();

            let endpoint = self.network.get_endpoint(player_id).unwrap();

            self.network.send_match_info(
                endpoint,
                MatchInfo {
                    client_id: *player_id,
                    players_state,
                    players_data: self.players_data.clone(),
                    bd_cards: self.bd_cards.clone(),
                    cards,
                    // start_cards: start_hand.remove(player_id).unwrap(),
                    // opp_start_cards: start_hand
                    //     .into_iter()
                    //     .map(|(k, v)| (k, v.into_iter().map(|(k, _)| k).collect()))
                    //     .collect(),
                },
            );
        }
        self.is_ready = true;
        println!("Match ready");
    }
    fn get_random_card_state(&mut self) -> CardState {
        self.bd_cards
            .get(Self::HC[rand::thread_rng().gen_range(0..10)])
            .unwrap()
            .clone() //shake deck
    }
    fn get_stats_from_bd(&self, hash_card: &HashCard) -> CardState {
        self.bd_cards.get(hash_card).unwrap().clone()
    }
    fn add_random_card(&mut self, player_id: &PlayerId) -> (CardId, HashCard) {
        let card_id = self.network.get_card_id();
        self.players_state
            .get_mut(player_id)
            .unwrap()
            .push_hand(card_id);
        let state = self.get_random_card_state();
        let hash_card = state.hash.clone();
        self.cards.insert(card_id, state);
        (card_id, hash_card)
    }
    // fn add_card(
    //     &mut self,
    //     player_id: &PlayerId,
    //     line_type: LineType,
    //     hash_card: &HashCard,
    // ) -> CardId {
    //     let card_id = self.network.get_card_id();
    //     self.players_state
    //         .get_mut(player_id)
    //         .unwrap()
    //         .add_card(card_id);
    //     self.cards
    //         .insert(card_id, self.get_stats_from_bd(hash_card));
    //     card_id
    // }
}

// // State::None => {
// // State::StartGame => {}
// // State::BeforeStep(id) => {}
// // State::PlayerStep(id) => {}
// // State::AfterStep(id) => {}
// // State::EndGame => {}
