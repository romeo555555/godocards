use std::collections::HashMap;

use crate::{
    gui::*,
    input::{self, *},
    layout::*,
    network::*,
    resources::*,
    store::*,
    store::*,
    utils::*,
};
use common::{game_match::MatchInfo, player::PlayerData};
use gdnative::prelude::{Input as GodoInput, *};
// enum State {
//     Auth,
//     Main,
//     Match,
//     Collections,
//     Exit,
// }

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct Game {
    //selected_card
    // selecting_card: SelectingCard,
    input: crate::input::Input,
    prefabs: Option<Prefabs>,
    game_match: Option<Match>,
}

#[methods]
impl Game {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Game builder is registered!");
    }
    fn new(_owner: &Node) -> Self {
        godot_print!("Game is created!");
        Self {
            // selecting_card: SelectingCard::new(),
            input: crate::input::Input::new(),
            prefabs: None,
            game_match: None,
        }
    }
    #[export]
    unsafe fn _ready(&mut self, owner: &Node) {
        logger::init(logger::Level::Info, logger::Output::File("log.txt")); //logger::Output::Stdout);
        log::info!("Game is ready!");
        // self.prefabs.init();
        godot_print!("Game is ready!");
    }
    #[export]
    unsafe fn _process(&mut self, owner: &Node, delta: f64) {
        // let entities = self
        //     .world
        //     .spawn_batch(vec![(LineT::A), (LineT::A), (LineT::B)])
        //     .collect::<Vec<Entity>>();
        // self.world.spawn().insert(LineT::A);
        // self.world.spawn().insert(LineT::A);
        // self.world.spawn().insert(LineT::B);

        // let mut query = self.world.query::<&LineT>();
        // godot_print!("{}", query.iter(&self.world).len());

        // for (mut position, velocity) in query.iter_mut(&mut world) {
        //     position.x += velocity.x;
        //     position.y += velocity.y;
        // }
        self.input.update(owner);
        if let Some(ref mut game_match) = self.game_match {
            log::info!("Cococclosing server");
            game_match.proceess(owner, &mut self.input)
        }
    }
    //Button does'nt work witch touuch
    #[export]
    fn _on_Match_pressed(&mut self, owner: &Node) {
        self.game_match = Some(Match::new(
            owner,
            PlayerData {
                name: "juja".to_owned(),
                deck_name: "deck".to_owned(),
                character_name: "avatarmini1".to_owned(),
                equipment_name: "items".to_owned(),
                factories_name: "builds".to_owned(),
                avatar: "avatar".to_owned(),
            },
            // vec_card: vec![
            //                  "unit1".to_owned(),
            //                  "unit2".to_owned(),
            //                  "unit3".to_owned(),
            //                  "wizard".to_owned(),
            //              ],
        ));
    }
    #[export]
    fn _on_Collections_pressed(&mut self, _owner: &Node) {}
    #[export]
    fn _on_Exit_pressed(&mut self, _owner: &Node) {}
}

pub enum MatchType {
    Match1x1,
    Match2x2,
}
pub struct Match {
    network: Network,
    layout: Layout,
    store: Store,
    gui: Gui,
}
impl Match {
    pub fn new(owner: &Node, player_data: PlayerData) -> Self {
        switch_visible(owner, 1i64);
        let (match_info, network) = Network::new(player_data);
        let (gui, store, layout) = Gui::new(owner, match_info);

        Self {
            network,
            layout,
            store,
            gui,
        }
    }
    pub fn proceess(&mut self, owner: &Node, input: &mut crate::input::Input) {
        let player_type = self.layout.contains_player(input);
        let player_state = self.store.get_player_state_from_type(&player_type);
        if let Some(action) = self.layout.input(input, player_type, player_state) {
            godot_print!("recive event : {:?}", action); //log
            dispatch(
                action,
                owner,
                input,
                &mut self.store,
                &mut self.gui,
                &mut self.network,
                &self.layout,
            );
        }
        if let Some(action) = self.network.receive_action() {
            godot_print!("recive event : {:?}", action); //log
            dispatch(
                action,
                owner,
                input,
                &mut self.store,
                &mut self.gui,
                &mut self.network,
                &self.layout,
            );
        };
        input.update_selected(&mut self.gui);
    }
}
// fn init(node: Node, world: &mut World) {
//     world.spawn().insert_bundle(PlayerBundle::new());
// }
