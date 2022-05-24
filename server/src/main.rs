use board::*;
use card::*;
// use common::CardId;
use mana::*;
mod board;
mod card;
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
    MatchCreated,
}
pub fn main() {
    GameMatch::default().run(
        Transport::FramedTcp,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3042),
    );
}
struct GameMatch {
    // config: Config,
    handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,
    history: Vec<Message>,
    queue_command: Vec<Message>,
    state: State,
    subscriptions: HashMap<Endpoint, PlayerId>,
    cards: CardsMap,
    players: HashMap<PlayerId, Player>,
    spawner: PlayerSpawner,
    is_ready: bool,
}
impl Default for GameMatch {
    fn default() -> Self {
        let (handler, listener) = node::split();
        Self {
            handler,
            listener: Some(listener),
            history: Vec::with_capacity(30),
            queue_command: Vec::with_capacity(30),
            state: State::None,
            subscriptions: HashMap::with_capacity(2), //HashSet
            players: HashMap::with_capacity(2),
            cards: CardsMap::new(),
            spawner: PlayerSpawner::default(),
            is_ready: false,
        }
    }
}
impl GameMatch {
    const NEEDED_PLAYER_FOR_START: usize = 2;
    pub fn run(&mut self, transport: Transport, addr: SocketAddr) {
        match self.handler.network().listen(transport, addr) {
            Ok((_id, real_addr)) => println!("Server running at {} by {}", real_addr, transport),
            Err(_) => return println!("Can not listening at {} by {}", addr, transport),
        }

        let listener = self.listener.take().unwrap();
        listener.for_each(move |event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_, _) => (), // Only generated at connect() calls.
                NetEvent::Accepted(endpoint, _listener_id) => {
                    // Only connection oriented protocols will generate this event
                    let player_id = self.spawner.new_id();
                    self.subscriptions.insert(endpoint, player_id);
                    println!(
                        "Client ({}) connected  player_id: {}",
                        endpoint.addr(),
                        player_id
                    );
                    // self.handler
                    //     .network()
                    //     .send(endpoint, &SerBin::serialize_bin(&player_id));
                }
                NetEvent::Message(endpoint, data) => {
                    if self.is_ready {
                        if let Result::<Message, DeBinErr>::Ok(msg) = DeBin::deserialize_bin(data) {
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
                        self.subscriptions.len()
                    );
                }
            },
            NodeEvent::Signal(signal) => match signal {
                // let subscriptions: Vec<Endpoint> = self.subscriptions.iter().cloned().collect();
                Signal::MatchCreated => {}
            },
        });
    }
    fn match_msg(&mut self, msg: Message) {
        match msg.event {
            Event::TakeCard(card_id) => {
                self.send_msg_for_all(msg.clone());
            }
            Event::FlipCard(card_id, hash_card) => {}
            Event::CastCardOnTabel(card_id) => {
                self.send_msg_for_all(msg.clone());
                // self.send_msg_for_all(Msg::build(msg.player_id, Event::FlipCard(card_id)));
            }
            Event::ChangeState(state) => {}
            Event::BackCardOnHand(card_id) => {}
            _ => {}
        }
        // msg.event
        // self.handler.network().send(endpoint, data);
    }
    fn add_player(&mut self, endpoint: Endpoint, player_handler: PlayerDataHandler) {
        self.players.insert(
            *self.subscriptions.get(&endpoint).unwrap(),
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
            self.handler.network().send(
                player.endpoint,
                &SerBin::serialize_bin(&MatchInfo {
                    client_id: *player_id,
                    players: map.clone(),
                    bd_cards: self.cards.get_bd(),
                    start_cards: start_hand.remove(&player_id).unwrap(),
                    opp_start_cards: start_hand
                        .into_iter()
                        .map(|(k, v)| (k, v.into_iter().map(|(k, _)| k).collect()))
                        .collect(),
                }),
            );
        }
        self.is_ready = true;
        println!("Match ready");
    }
    fn send_msg(self, endpoint: Endpoint, msg: Message) {
        self.handler
            .network()
            .send(endpoint, &SerBin::serialize_bin(&msg));
    }
    fn send_msg_for_all(&mut self, msg: Message) {
        self.subscriptions.keys().for_each(|endpoint| {
            self.handler
                .network()
                .send(*endpoint, &SerBin::serialize_bin(&msg));
        });
    }
}
// struct GameMatch {
//     board: Board,
//     // players: HashMap<Endpoint, Player>,
//     state: State,
// }
// impl GameMatch {
//     // fn new(&mut self)->Self {
//     //     Self{board:Board::new()}
//     // }
//     fn add_player(&mut self, endpoint: Endpoint, player_handler: PlayerDataHandler) {
//         self.board
//             .players
//             .push(Player::new(endpoint, player_handler));
//     }
//     fn is_full(&self, len: usize) -> bool {
//         Board::is_full(len)
//     }
//     fn is_ready(&self) -> bool {
//         self.board.is_ready()
//     }
//     pub fn match_event(&mut self, event: Event) -> Option<Event> {
//         match event {
//             Event::ChangeState(state) => {}
//             Event::TakeCard => {
//                 // if State::PlayerStep(player_id) = state{
//                 //     if player_id == endpoint{

//                 //     }
//                 // }
//             }
//             Event::CastCardOnTabel(card_id) => {}
//             Event::BackCardOnHand(card_id) => {}
//         }
//         None
//     }
// }
// // State::None => {
// // State::StartGame => {}
// // State::BeforeStep(id) => {}
// // State::PlayerStep(id) => {}
// // State::AfterStep(id) => {}
// // State::EndGame => {}
