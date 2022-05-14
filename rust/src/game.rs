use crate::boards::*;
use crate::input::*;
use crate::network::*;
use crate::player::*;
use crate::rendering::*;
use crate::resources::*;
use crate::utils::*;
use gdnative::api::*;
use gdnative::prelude::*;
use message_io::node::NodeTask;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

enum State {
    Auth,
    Main,
    Match,
    Collections,
    Exit,
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct Game {
    name: String,
    rendering: Rendering,
    resources: Resources,
    state: State,
    gs: GameSetting,
    game_match: Option<Match>,
    // main_menu: Option<Node>,
    // game: Option<Node>,
    // collections: Option<Node>,
}

#[methods]
impl Game {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Game builder is registered!");
    }
    fn new(owner: &Node) -> Self {
        let resources = Resources::default();
        let rendering = Rendering::default();
        let gs = GameSetting::default();

        let size = OS::godot_singleton().get_screen_size(-1);
        godot_print!("{} --- {} : OS screen_size", size.x, size.y);
        godot_print!("Game is created!");
        Self {
            name: "".to_string(),
            game_match: None,
            rendering,
            resources,
            state: State::Auth,
            gs,
        }
    }
    #[export]
    unsafe fn _ready(&mut self, owner: &Node) {
        self.resources.load();

        self.name = "Game".to_string();
        let size = owner
            .cast::<CanvasItem>()
            .map(|node| node.get_viewport_rect())
            .map(|viewport| {
                godot_print!(
                    "________{}-{}_______ is screen pos! //// ________{}-{}_______ is screen size!",
                    viewport.position.x,
                    viewport.position.y,
                    viewport.size.x,
                    viewport.size.y,
                );
                viewport.size
            })
            .unwrap();
        self.gs = GameSetting::new(size, vec2(0., 0.));

        godot_print!("{} is ready!", self.name);
    }
    #[export]
    unsafe fn _process(&mut self, owner: &Node, delta: f64) {
        if let Some(ref mut game_match) = self.game_match {
            game_match.process(owner, &mut self.rendering, &mut self.resources, &self.gs);
        }
        // self.curent_scene = match self.curent_scene {
        // match self.curent_scene {
        //     Scene::Auth => {}
        //     Scene::Main => {}
        //     Scene::Match => {}
        //     Scene::Collections => {}
        //     _ => {
        //         panic!("exit");
        //     }
        // }
    }
    fn match_pressed(&mut self, owner: &Node, id: PlayerId) {
        self.state = State::Match;
        switch_visible(owner, 1i64);
        // switch_visible(owner, 2i64);
        //other data not the same as other player
        //TODO: CLIENT!!!
        let player_client = PlayerDataHandler {
            id: PlayerId::default(),
            character_name: "avatarmini1".to_owned(),
            deck_name: "deck".to_owned(),
            items_name: "items".to_owned(),
            builds_name: "builds".to_owned(),
            avatar: "avatar".to_owned(),
            data: PlayerData {
                name: "afkdsfv".to_owned(),
                vec_card: Vec::with_capacity(30),
                character: "avatarmini1".to_owned(),
            },
        };

        let mut network = Network::new(player_client);

        let match_info = loop {
            if let Message::MatchInfo(match_info) = network.event_queue.receive() {
                break match_info;
            }
        };

        self.game_match = Some(Match::new(
            owner,
            // thread,
            // task,
            // players_rx.recv().expect("player data helper error"),
            // network_rx,
            // message_tx,
            network,
            match_info,
            &mut self.resources,
            &self.gs,
        ));
    }

    //Button does'nt work witch touuch
    #[export]
    fn _on_Match_pressed(&mut self, owner: &Node) {
        self.match_pressed(owner, 0);
    }
    #[export]
    fn _on_Collections_pressed(&mut self, _owner: &Node) {
        self.state = State::Collections;
    }
    #[export]
    fn _on_Exit_pressed(&mut self, _owner: &Node) {
        self.state = State::Exit;
    }
}
fn switch_visible(owner: &Node, idx: i64) {
    let node = unsafe { owner.get_child(idx).expect("Missing node").assume_safe() }
        .cast::<CanvasItem>()
        .expect("Node should cast to CanvasItem");
    node.set_visible(!node.is_visible());
}

struct Match {
    // type_macth: TypeMacth
    board: Board,
    // thread: JoinHandle<()>,
    // task: NodeTask,
    network: Network,
}
impl Match {
    fn new(
        owner: &Node,
        // thread: JoinHandle<()>,
        // task: NodeTask,
        // player_remote: PlayerDataHandler,
        // player_client: PlayerDataHandler,
        // players: HashMap<PlayerId, PlayerDataHandler>,
        // network_rx: Receiver<Message>,
        // messaage_tx: Sender<Message>,
        network: Network,
        match_info: MatchInfo,
        res: &mut Resources,
        gs: &GameSetting,
    ) -> Self {
        let board = Board::new(
            owner,
            // players,
            // player_remote,
            // player_client,
            // players.remove(&1).unwrap(),
            // players.remove(&0).unwrap(),
            // network_rx,
            // messaage_tx,
            match_info, res, gs,
        );
        Self { board, network } // thread
    }
    fn process(
        &mut self,
        owner: &Node,
        rendering: &mut Rendering,
        resources: &mut Resources,
        gs: &GameSetting,
    ) {
        let input = Input::godot_singleton();
        let sense = Sense::new(
            owner
                .cast::<CanvasItem>()
                .map(|node| node.get_global_mouse_position())
                // .map(|viewport| {
                //     godot_print!("___vec2({}-{})___mouse_pos", viewport.x, viewport.y);
                //     viewport
                // })
                .unwrap(),
            resources.card_size,
            Input::is_action_just_pressed(input, "mouse_button", false),
            Input::is_action_just_released(input, "mouse_button", false),
        );
        self.board
            .network_event(owner, &mut self.network, resources);
        self.board
            .input_handler(rendering, sense, gs, owner, resources);
        rendering.pre_draw();
        rendering.draw(owner, resources);
        rendering.after_draw(resources, sense);
    }
}
#[derive(Default)]
pub struct GameSetting {
    pub screen_size: Vec2,
    pub screen_width: f32,
    pub screen_height: f32,
    pub screen_rect: Rect,
    pub card_size: Vec2,
}
impl GameSetting {
    // fn new(w: f32, h: f32) -> Self {
    // fn new(screen_size: Vec2) -> Self {
    fn new(screen_size: Vec2, card_size: Vec2) -> Self {
        Self {
            screen_rect: Rect::new(0., 0., screen_size.x, screen_size.y),
            screen_size,
            card_size,
            screen_width: screen_size.x,
            screen_height: screen_size.y,
        }
    }
    pub fn is_up_side(&self, mouse_y: f32) -> bool {
        self.screen_size.y > mouse_y
    }
}
