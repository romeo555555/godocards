use crate::*;
use gdnative::prelude::godot_print;
use message_io::events::EventReceiver;
use message_io::network::{Endpoint, NetEvent, Transport};
// use message_io::node::{
//     self, NodeHandler, NodeTask, StoredNetEvent as NetEvent, StoredNodeEvent as NodeEvent,
// };
// use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self, NodeEvent, NodeHandler, NodeListener, NodeTask};
use nanoserde::{DeBin, DeBinErr, SerBin};
use std::collections::{HashMap, HashSet};
use std::result;
use std::time::Duration;
pub mod common;
// pub mod event;
pub use common::*;
// pub use common::*;
// pub use event::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
enum Signal {
    // MatchFound(MatchInfo),
    // CreateMatch(MatchInfo),
    Send(Message),
    // Greet,
}
pub struct Network {
    // transport: Transport,
    // remote_addr: String,
    // config: Config,
    // server_id: Endpoint,
    // local_addr: SocketAddr,
    // handler: NodeHandler<Signal>,
    // receiver: EventReceiver<NodeEvent<Signal>>,
    // listener: NodeListener<Signal>,
    // _task: NodeTask,
    // listener: Option<NodeListener<Signal>>,
    // subscriptions: HashSet<Endpoint>,
    // pub network_rx: Receiver<Message>,
    // pub message_tx: Sender<Message>,
    pub event_queue: EventReceiver<Message>,
    _server: ServerProxy,
}
impl Network {
    pub fn new(player_client: PlayerDataHandler) -> Self {
        let event_queue = EventReceiver::default();

        let sender = event_queue.sender().clone();
        let server = ServerProxy::new(player_client, move |server_event| sender.send(server_event)); //?
        Self {
            event_queue,
            _server: server,
        }
    }
    pub fn call(&mut self, msg: Message) {
        self._server.call(msg);
    }
    // pub fn network_event(&mut self, board: &mut Board, owner: &Node, resources: &mut Resources) {
    //     board
    //         .players
    //         .values_mut()
    //         .for_each(|player| player.update_position(resources));

    //     let msg = self.event_queue.receive();
    //     match msg.event {
    //         Event::TakeCard => {
    //             // self.player_client
    //             //     .add_card_on_hand(res.create_card(owner));
    //             // let card_id = res.create_card(card_name.clone());
    //             // let side_player = self.get_side_player(commmand.player);
    //             // match commmand.line {
    //             //     LineType::Hand => side_player.hand.add_card(card_id),
    //             //     LineType::Tabel => side_player.tabel.add_card(card_id),
    //             //     _ => {}
    //             // }

    //             board
    //                 .players
    //                 .get_mut(board.client_id)
    //                 .expect("player_client not found")
    //                 .add_card_on_hand(create::card(owner, resources, 10000));
    //         }
    //         Event::CastCardOnTabel(ref card_id) => {
    //             // self.side_client.cast_on_tabel(card_id)
    //         } //match player
    //         Event::BackCardOnHand(card_id) => {
    //             // self.side_client.back_on_hand(card_id)
    //         }
    //         _ => {}
    //     }
    //     self.history.push(msg);
    // }
}

pub struct ServerProxy {
    node: NodeHandler<Signal>,
    node_task: NodeTask,
}

impl ServerProxy {
    pub fn new(
        player_client: PlayerDataHandler,
        event_callback: impl Fn(Message) + Send + 'static,
    ) -> ServerProxy {
        let (handler, listener) = node::split();

        let transport = Transport::FramedTcp;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3042);
        let (server_id, local_addr) = handler.network().connect(transport, addr).unwrap();
        // let mut connection = ServerConnection::new(node.clone());
        println!("Client addres at transpot: {} addr: {}", transport, addr,);

        let node = handler.clone();
        let mut match_create = false;

        let node_task = listener.for_each_async(move |event| {
            // connection.process_event(event, |server_event| event_callback(server_event));
            match event {
                NodeEvent::Signal(signal) => {
                    if let Signal::Send(msg) = signal {
                        handler
                            .network()
                            .send(server_id, &SerBin::serialize_bin(&msg));
                    }
                }
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Connected(_, established) => {
                        if established {
                            godot_print!("Connected to server at {}", server_id.addr(),);
                            godot_print!("Client identified by local port: {}", local_addr.port());
                            handler
                                .network()
                                .send(server_id, &SerBin::serialize_bin(&player_client));
                        } else {
                            godot_print!("Can not connect to server");
                        }
                    }
                    NetEvent::Accepted(_, _) => unreachable!(),
                    NetEvent::Message(_, data) => {
                        if match_create {
                            if let Result::<Message, DeBinErr>::Ok(msg) =
                                DeBin::deserialize_bin(data)
                            {
                                if let Message::Message(_) = msg {
                                    event_callback(msg);
                                }
                            }
                        } else if let Result::<MatchInfo, DeBinErr>::Ok(match_info) =
                            DeBin::deserialize_bin(data)
                        {
                            match_create = true;
                            event_callback(Message::MatchInfo(match_info));
                        }
                    }
                    NetEvent::Disconnected(_) => {
                        godot_print!("Server is disconnected");
                        handler.stop();
                    }
                },
            }
        });
        ServerProxy { node, node_task }
    }
    pub fn call(&mut self, msg: Message) {
        self.node.signals().send(Signal::Send(msg));
    }

    // pub fn api(&mut self) -> ServerApi {
    //     ServerApi {
    //         node: self.node.clone(),
    //     }
    // }
}
impl Drop for ServerProxy {
    fn drop(&mut self) {
        self.node.stop();
        self.node_task.wait();
    }
}
// pub struct ServerApi {
//     node: NodeHandler<Signal>,
// }
// impl ServerApi {
//     pub fn call(&mut self, msg: Message) {
//         self.node.signals().send(Signal::Send(msg));
//     }
// }
// pub fn connect(mut player_client: PlayerDataHandler) {
//     let transport = Transport::FramedTcp;
//     let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3042);
//     println!("Client addres at transpot: {} addr: {}", transport, addr,);

//     let (match_tx, match_rx) = mpsc::channel::<MatchInfo>(); //HashMap<PlayerId, PlayerDataHandler>>();
//     let (network_tx, network_rx) = mpsc::channel::<Message>();
//     let (message_tx, message_rx) = mpsc::channel::<Message>();

//     let (handler, listener) = node::split::<()>();
//     let (server_id, local_addr) = handler.network().connect(transport, addr).unwrap();
//     let (_task, mut receiver) = listener.enqueue();

//     // let mut match_state = State::None;
//     // thread::spawn(move || {
//     //     let mut match_create = false;
//     //     loop {
//     //         if let Some(NodeEvent::Network(event)) = receiver.try_receive() {
//     //             godot_print!("0000");
//     //             match event {
//     //                 NetEvent::Connected(_, established) => {
//     //                     if established {
//     //                         godot_print!("Connected to server at {}", server_id.addr(),);
//     //                         godot_print!(
//     //                             "Client identified by local port: {}",
//     //                             local_addr.port()
//     //                         );
//     //                         handler
//     //                             .network()
//     //                             .send(server_id, &SerBin::serialize_bin(&player_client));
//     //                     } else {
//     //                         godot_print!("Can not connect to server");
//     //                     }
//     //                 }
//     //                 NetEvent::Accepted(_, _) => unreachable!(),
//     //                 NetEvent::Message(_, data) => {
//     //                     if match_create {
//     //                         for msg in message_rx.try_iter() {
//     //                             // for msg in message_rx.try_iter() {
//     //                             // if let Ok(msg) = message_rx.try_recv() {
//     //                             handler
//     //                                 .network()
//     //                                 .send(server_id, &SerBin::serialize_bin(&msg));
//     //                         }
//     //                         if let Result::<Message, DeBinErr>::Ok(msg) =
//     //                             DeBin::deserialize_bin(&data)
//     //                         {
//     //                             network_tx.send(msg).expect("ddd");
//     //                         }
//     //                     } else if let Result::<MatchInfo, DeBinErr>::Ok(match_ifo) =
//     //                         DeBin::deserialize_bin(&data)
//     //                     {
//     //                         match_tx.send(match_ifo).expect("dddd");
//     //                         //TODO: if player_count > 2 ??
//     //                         match_create = true;
//     //                     }
//     //                 }
//     //                 NetEvent::Disconnected(_) => {
//     //                     godot_print!("Server is disconnected");
//     //   handler.stop();
//     //                 }
//     //             }
//     //         }
//     //     }
//     // });
//     // (
//     //     Self {
//     //         server_id,
//     //         local_addr,
//     //         // handler,
//     //         // listener,
//     //         _task,
//     //         // receiver,
//     //         network_rx,
//     //         message_tx,
//     //     },
//     //     match_rx.recv().expect("recv errr players from net"),
//     // )
// }

// mod message {
//     use nanoserde::{DeBin, SerBin};

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct State(pub [u8; 4]);
//     impl State {
//         pub const OPCODE: i32 = 1;
//     }

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct Damage {
//         pub target: String,
//         pub direction: bool,
//     }
//     impl Damage {
//         pub const OPCODE: i32 = 2;
//     }

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct SpawnItem {
//         pub id: u32,
//         pub x: u16,
//         pub y: u16,
//         pub item_type: u8,
//     }
//     impl SpawnItem {
//         pub const OPCODE: i32 = 4;
//     }

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct DeleteItem {
//         pub id: u32,
//     }
//     impl DeleteItem {
//         pub const OPCODE: i32 = 5;
//     }

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct Ready;
//     impl Ready {
//         pub const OPCODE: i32 = 6;
//     }

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct Idle;
//     impl Idle {
//         pub const OPCODE: i32 = 7;
//     }

//     #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
//     pub struct StartGame;
//     impl StartGame {
//         pub const OPCODE: i32 = 8;
//     }
// }
