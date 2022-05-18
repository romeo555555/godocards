use crate::input::*;
use crate::matchs::*;
use crate::network::*;
use crate::player::*;
use crate::resources::*;
use crate::systems::*;
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
        self.name = "Game".to_string();
        self.resources
            .load_prefabs_and_config(Config::new(owner, vec2(150., 180.)));
        godot_print!("{} is ready!", self.name);
    }
    #[export]
    unsafe fn _process(&mut self, owner: &Node, delta: f64) {
        if let Some(ref mut game_match) = self.game_match {
            game_match.input(owner, &mut self.resources);
            game_match.event(owner, &mut self.resources);
        }
    }
    //Button does'nt work witch touuch
    #[export]
    fn _on_Match_pressed(&mut self, owner: &Node) {
        self.game_match = Some(Match::Match1x1(Match1x1::new(
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
                    vec_card: Vec::with_capacity(30),
                    character: "avatarmini1".to_owned(),
                },
            },
        )));
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
    pub card_size: Vec2,
}
impl Config {
    // fn new(w: f32, h: f32) -> Self {
    // fn new(screen_size: Vec2) -> Self {
    // fn new(screen_size: Vec2, card_size: Vec2) -> Self {
    fn new(owner: &Node, card_size: Vec2) -> Self {
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
        }
    }
    pub fn is_up_side(&self, mouse_y: f32) -> bool {
        self.screen_size.y > mouse_y
    }
}
