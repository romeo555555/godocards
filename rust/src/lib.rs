mod boards;
mod game;
mod input;
mod network;
mod player;
// pub mod player::HashId;
mod rendering;
mod resources;
mod utils;

use crate::player::components::card::*;
use boards::*;
use game::*;
use input::*;
use network::*;
use player::*;
use rendering::*;
use resources::*;
use utils::*;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
