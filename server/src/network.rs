use crate::*;

pub struct Network {
    handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,
    subscriptions: HashMap<Endpoint, PlayerId>,
    spawner: Spawner,
    history: Vec<ServerMessage>,
}
impl Network {
    pub fn new() -> Self {
        let (handler, listener) = node::split();
        Self {
            handler,
            listener: Some(listener),
            subscriptions: HashMap::with_capacity(2), //HashSet
            spawner: Spawner::default(),
            history: Vec::with_capacity(30),
        }
    }
    pub fn take_listener(
        &mut self,
        transport: Transport,
        addr: SocketAddr,
    ) -> NodeListener<Signal> {
        match self.handler.network().listen(transport, addr) {
            Ok((_id, real_addr)) => println!("Server running at {} by {}", real_addr, transport),
            Err(_) => panic!("Can not listening at {} by {}", addr, transport),
        }
        self.listener.take().unwrap()
    }
    pub fn add_sub(&mut self, endpoint: Endpoint) {
        let player_id = self.spawner.new_player_id();
        self.subscriptions.insert(endpoint, player_id);
        println!(
            "Client ({}) connected  player_id: {}",
            endpoint.addr(),
            player_id
        );
    }
    pub fn get_endpoint(&mut self, player_id: &PlayerId) -> Option<Endpoint> {
        for (endpoint, id) in self.subscriptions.iter() {
            if id == player_id {
                return Some(*endpoint);
            }
        }
        None
    }
    pub fn get_sub_id(&mut self, endpoint: Endpoint) -> PlayerId {
        self.subscriptions.get(&endpoint).unwrap().clone()
    }
    pub fn get_card_id(&mut self) -> CardId {
        self.spawner.new_card_id()
    }
    pub fn send_match_info(&self, endpoint: Endpoint, match_info: MatchInfo) {
        self.handler
            .network()
            .send(endpoint, &SerBin::serialize_bin(&match_info));
    }
    pub fn send_msg(&self, endpoint: Endpoint, msg: &ServerMessage) {
        self.handler
            .network()
            .send(endpoint, &SerBin::serialize_bin(msg));
    }
    pub fn send_msg_for_all(&mut self, msg: &ServerMessage) {
        self.subscriptions.keys().for_each(|endpoint| {
            self.handler
                .network()
                .send(*endpoint, &SerBin::serialize_bin(msg));
        });
    }
}
#[derive(Default)]
pub struct Spawner {
    id_counter_player: PlayerId,
    id_counter_card: CardId,
}
impl Spawner {
    pub fn new_player_id(&mut self) -> PlayerId {
        self.id_counter_player += 1;
        self.id_counter_player
    }
    pub fn new_card_id(&mut self) -> CardId {
        self.id_counter_card += 1;
        self.id_counter_card
    }
}
