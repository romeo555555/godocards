use crate::gui::*;
use crate::input::*;
use crate::matchmaking::*;
use crate::network::*;
use crate::resources::*;
use crate::utils::*;
use gdnative::api::*;
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
    name: String,
    resources: Resources,
    // rendering: Rendering,
    // input: Input,
    game_match: Option<Match>,
}

#[methods]
impl Game {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Game builder is registered!");
    }
    fn new(_owner: &Node) -> Self {
        let size = OS::godot_singleton().get_screen_size(-1);
        godot_print!("{} --- {} : OS screen_size", size.x, size.y);
        godot_print!("Game is created!");
        Self {
            name: "".to_string(),
            resources: Resources::default(),
            game_match: None,
        }
    }
    #[export]
    unsafe fn _ready(&mut self, owner: &Node) {
        logger::init(logger::Level::Info, logger::Output::File("log.txt")); //logger::Output::Stdout);
        log::info!("Closing server");
        self.name = "Game".to_string();
        self.resources
            .load_prefabs_and_config(Config::new(owner, vec2(150., 180.), vec2(10., 0.)));
        godot_print!("{} is ready!", self.name);
    }
    #[export]
    unsafe fn _process(&mut self, owner: &Node, delta: f64) {
        if let Some(ref mut game_match) = self.game_match {
            // game_match.draw(owner, &mut self.resources);
            game_match.input(owner, &mut self.resources);
            game_match.event(owner, &mut self.resources);
            log::info!("Cococclosing server");
        }
    }
    //Button does'nt work witch touuch
    #[export]
    fn _on_Match_pressed(&mut self, owner: &Node) {
        self.game_match = Some(Match::new(
            owner,
            &mut self.resources,
            PlayerDataHandler {
                id: PlayerId::default(),
                character_name: "avatarmini1".to_owned(),
                deck_name: "deck".to_owned(),
                items_name: "items".to_owned(),
                builds_name: "builds".to_owned(),
                avatar: "avatar".to_owned(),
                data: PlayerData {
                    name: "afkdsfv".to_owned(),
                    vec_card: vec![
                        "unit1".to_owned(),
                        "unit2".to_owned(),
                        "unit3".to_owned(),
                        "wizard".to_owned(),
                    ],
                    character: "avatarmini1".to_owned(),
                },
            },
        ));
    }
    #[export]
    fn _on_Collections_pressed(&mut self, _owner: &Node) {}
    #[export]
    fn _on_Exit_pressed(&mut self, _owner: &Node) {}
}

#[derive(Default)]
pub struct Config {
    pub screen_size: Vec2,
    pub screen_width: f32,
    pub screen_height: f32,
    pub screen_rect: Rect,
    pub card_indent: Vec2,
    pub card_size: Vec2,
}
impl Config {
    fn new(owner: &Node, card_size: Vec2, card_indent: Vec2) -> Self {
        let screen_size = owner
            .cast::<CanvasItem>()
            .map(|node| node.get_viewport_rect())
            .map(|viewport| {
                godot_print!(
                    "_{}-{}_ is screen pos! //// _{}-{}_ is screen size!",
                    viewport.position.x,
                    viewport.position.y,
                    viewport.size.x,
                    viewport.size.y,
                );
                viewport.size
            })
            .unwrap();
        Self {
            screen_rect: Rect::new(0., 0., screen_size.x, screen_size.y),
            screen_size,
            card_size,
            screen_width: screen_size.x,
            screen_height: screen_size.y,
            card_indent,
        }
    }
    pub fn is_up_side(&self, mouse_y: f32) -> bool {
        self.screen_size.y > mouse_y
    }
}
