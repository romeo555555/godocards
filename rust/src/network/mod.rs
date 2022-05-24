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
    pub fn new(player_client: PlayerDataHandler) -> (Self, EventReceiver<MatchInfo>) {
        let event_queue = EventReceiver::default();
        let match_reciver = EventReceiver::default();

        let sender = event_queue.sender().clone();
        let match_sender = match_reciver.sender().clone();
        let server = ServerProxy::new(
            player_client,
            move |server_event| sender.send(server_event),
            move |server_event| match_sender.send(server_event),
        ); //?
        (
            Self {
                event_queue,
                _server: server,
            },
            match_reciver,
        )
    }
    pub fn call(&mut self, msg: Message) {
        self._server.call(msg);
    }
}

pub struct ServerProxy {
    node: NodeHandler<Signal>,
    node_task: NodeTask,
}

impl ServerProxy {
    pub fn new(
        player_client: PlayerDataHandler,
        event_callback: impl Fn(Message) + Send + 'static,
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
                            if let Result::<Message, DeBinErr>::Ok(msg) =
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
