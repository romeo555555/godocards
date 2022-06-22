use crate::*;
use common::game_match::MatchInfo;
use common::player::{PlayerData, PlayerId};
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
extern crate common;
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
    Send(ClientMessage),
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
    event_queue: EventReceiver<ServerMessage>,
    _server: ServerProxy,
    pub client_id: PlayerId,
    event: Vec<ClientMessage>,
}
impl Network {
    pub fn new(player_client: PlayerData) -> (ServerApi, MatchInfo, Self) {
        let event_queue = EventReceiver::default();
        let mut match_reciver = EventReceiver::default(); //?mut

        let sender = event_queue.sender().clone();
        let match_sender = match_reciver.sender().clone();
        let mut server = ServerProxy::new(
            player_client,
            move |server_event| sender.send(server_event),
            move |server_event| match_sender.send(server_event),
        ); //?

        let match_info = match_reciver.receive();
        let client_id = match_info.client_id;
        (
            server.api(client_id),
            match_info,
            Self {
                event_queue,
                _server: server,
                client_id,
                event: Vec::with_capacity(5),
            },
        )
    }
    pub fn send_msg(&mut self, action: ClientAction) {
        godot_print!("send event : {:?}", action);
        // self.event.push(ClientMessage::build(action));
        self._server.call(ClientMessage { action });
    }

    pub fn receive_event(&mut self) -> Option<ServerMessage> {
        // if let Some(msg) = self.event.pop() {
        //     godot_print!("Message Event");
        //     self._server.call(msg);
        // }
        self.event_queue.try_receive()
    }
    pub fn receive_action(&mut self) -> Option<Action> {
        // if let Some(msg) = self.event.pop() {
        //     godot_print!("Message Event");
        //     self._server.call(msg);
        // }
        self.event_queue
            .try_receive()
            .map(|msg| Action::Server(msg))
    }
}

pub struct ServerProxy {
    node: NodeHandler<Signal>,
    node_task: NodeTask,
}

impl ServerProxy {
    pub fn new(
        player_client: PlayerData,
        event_callback: impl Fn(ServerMessage) + Send + 'static,
        match_info_callback: impl Fn(MatchInfo) + Send + 'static,
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
                            if let Result::<ServerMessage, DeBinErr>::Ok(msg) =
                                DeBin::deserialize_bin(data)
                            {
                                event_callback(msg);
                            }
                        } else if let Result::<MatchInfo, DeBinErr>::Ok(match_info) =
                            DeBin::deserialize_bin(data)
                        {
                            match_create = true;
                            match_info_callback(match_info);
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
    pub fn call(&mut self, msg: ClientMessage) {
        self.node.signals().send(Signal::Send(msg));
    }

    pub fn api(&mut self, player_id: PlayerId) -> ServerApi {
        ServerApi {
            node: self.node.clone(),
            player_id,
        }
    }
}
impl Drop for ServerProxy {
    fn drop(&mut self) {
        self.node.stop();
        self.node_task.wait();
    }
}
pub struct ServerApi {
    node: NodeHandler<Signal>,
    player_id: PlayerId,
}
impl ServerApi {
    // pub fn call(&mut self, msg: ClientMessage) {
    //     self.node.signals().send(Signal::Send(msg));
    // }
    pub fn send(&mut self, action: ClientAction) {
        self.node
            .signals()
            .send(Signal::Send(ClientMessage { action }));
    }
}
