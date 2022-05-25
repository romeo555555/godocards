use card::*;
use player::*;
// use common::CardId;
use mana::*;
mod card;
mod player;
// mod common;
mod mana;
use common::*;
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
    network: Network,
    state: State,
    cards: CardsMap,
    players: HashMap<PlayerId, Player>,
    is_ready: bool,
}
impl Default for GameMatch {
    fn default() -> Self {
        Self {
            network: Network::new(),
            state: State::None,
            players: HashMap::with_capacity(2),
            cards: CardsMap::new(),
            is_ready: false,
        }
    }
}
impl GameMatch {
    const NEEDED_PLAYER_FOR_START: usize = 2;
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
                            if let Result::<Message, DeBinErr>::Ok(msg) =
                                DeBin::deserialize_bin(data)
                            {
                                println!("take event");
                                self.match_msg(msg);
                            }
                        } else if let Result::<PlayerDataHandler, DeBinErr>::Ok(player) =
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
    fn match_msg(&mut self, msg: Message) {
        match msg.event {
            Event::TakeCard(card_id) => {
                self.network.send_msg_for_all(&msg);
            }
            Event::FlipCard(card_id, hash_card) => {}
            Event::CastCardOnTabel(card_id) => {
                self.network.send_msg_for_all(&msg);
                // self.send_msg_for_all(Msg::build(msg.player_id, Event::FlipCard(card_id)));
            }
            Event::ChangeState(state) => {}
            Event::BackCardOnHand(card_id) => {}
            _ => {}
        }
    }
    fn add_player(&mut self, endpoint: Endpoint, player_handler: PlayerDataHandler) {
        self.players.insert(
            self.network.get_sub(endpoint),
            Player::new(endpoint, player_handler),
        );

        if self.players.len() == GameMatch::NEEDED_PLAYER_FOR_START {
            self.create_match();
        }
    }
    fn create_match(&mut self) {
        let map: HashMap<PlayerId, PlayerDataHandler> = self
            .players
            .iter()
            .map(|(id, player)| (*id, player.player_handler.clone()))
            .collect();

        let mut init_card: HashMap<PlayerId, Vec<(CardId, HashCard)>> =
            HashMap::with_capacity(self.players.len());

        for (player_id, player) in self.players.iter_mut() {
            let mut vec = Vec::with_capacity(3);
            for _ in 0..3 {
                let hash_card = player.get_random_card_hash();
                let card_id = self.cards.add_card(hash_card.clone());
                player.add_card_hand(card_id);
                vec.push((card_id, hash_card));
            }
            init_card.insert(*player_id, vec);
        }
        for (player_id, player) in &self.players {
            let mut start_hand = init_card.clone();
            self.network.send_match_info(
                player.endpoint,
                MatchInfo {
                    client_id: *player_id,
                    players: map.clone(),
                    bd_cards: self.cards.get_bd(),
                    start_cards: start_hand.remove(&player_id).unwrap(),
                    opp_start_cards: start_hand
                        .into_iter()
                        .map(|(k, v)| (k, v.into_iter().map(|(k, _)| k).collect()))
                        .collect(),
                },
            );
        }
        self.is_ready = true;
        println!("Match ready");
    }
}

// // State::None => {
// // State::StartGame => {}
// // State::BeforeStep(id) => {}
// // State::PlayerStep(id) => {}
// // State::AfterStep(id) => {}
// // State::EndGame => {}
struct Network {
    handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,
    subscriptions: HashMap<Endpoint, PlayerId>,
    spawner: PlayerSpawner,
    history: Vec<Message>,
}
impl Network {
    fn new() -> Self {
        let (handler, listener) = node::split();
        Self {
            handler,
            listener: Some(listener),
            subscriptions: HashMap::with_capacity(2), //HashSet
            spawner: PlayerSpawner::default(),
            history: Vec::with_capacity(30),
        }
    }
    fn take_listener(&mut self, transport: Transport, addr: SocketAddr) -> NodeListener<Signal> {
        match self.handler.network().listen(transport, addr) {
            Ok((_id, real_addr)) => println!("Server running at {} by {}", real_addr, transport),
            Err(_) => panic!("Can not listening at {} by {}", addr, transport),
        }
        self.listener.take().unwrap()
    }
    fn add_sub(&mut self, endpoint: Endpoint) {
        let player_id = self.spawner.new_id();
        self.subscriptions.insert(endpoint, player_id);
        println!(
            "Client ({}) connected  player_id: {}",
            endpoint.addr(),
            player_id
        );
    }
    fn get_sub(&mut self, endpoint: Endpoint) -> PlayerId {
        self.subscriptions.get(&endpoint).unwrap().clone()
    }
    fn send_match_info(&self, endpoint: Endpoint, match_info: MatchInfo) {
        self.handler
            .network()
            .send(endpoint, &SerBin::serialize_bin(&match_info));
    }
    fn send_msg(&self, endpoint: Endpoint, msg: &Message) {
        self.handler
            .network()
            .send(endpoint, &SerBin::serialize_bin(msg));
    }
    fn send_msg_for_all(&mut self, msg: &Message) {
        self.subscriptions.keys().for_each(|endpoint| {
            self.handler
                .network()
                .send(*endpoint, &SerBin::serialize_bin(msg));
        });
    }
}
