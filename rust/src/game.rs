use std::collections::HashMap;

use crate::{
    gui::*, input_action::*, network::*, resources::*, selecting::*, store::*, store::*, utils::*,
};
use common::{game_match::MatchInfo, player::PlayerData};
use gdnative::prelude::*;
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
    selecting_card: SelectingCard,
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
            selecting_card: SelectingCard::new(),
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
        if let Some(ref mut game_match) = self.game_match {
            log::info!("Cococclosing server");
            game_match.proceess(
                owner
                    .cast::<CanvasItem>()
                    .map(|node| node.get_global_mouse_position())
                    .unwrap(),
                owner,
                &mut self.selecting_card,
            )
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

pub struct Match {
    network: Network,
    store: Store,
    gui: Gui,
}
impl Match {
    pub fn new(owner: &Node, player_data: PlayerData) -> Self {
        switch_visible(owner, 1i64);
        let (server_api, match_info, network) = Network::new(player_data);
        let (gui, store) = Gui::new(owner, match_info, server_api);

        Self {
            network,
            store,
            gui,
        }
    }
    pub fn proceess(&mut self, mouse_pos: Vec2, owner: &Node, selected_card: &mut SelectingCard) {
        if let Some(action) = self.gui.input(
            Sense::new(mouse_pos),
            self.store.get_players_state_map(),
            selected_card,
        ) {
            godot_print!("recive event : {:?}", action); //log
            dispatch(
                action,
                owner,
                &mut self.store,
                &mut self.gui,
                &mut self.network,
                selected_card,
            );
        }
        if let Some(action) = self.network.receive_action() {
            godot_print!("recive event : {:?}", action); //log
            dispatch(
                action,
                owner,
                &mut self.store,
                &mut self.gui,
                &mut self.network,
                selected_card,
            );
        };
        selected_card.update_selected(mouse_pos, &mut self.gui);
    }
}
// fn init(node: Node, world: &mut World) {
//     world.spawn().insert_bundle(PlayerBundle::new());
// }
