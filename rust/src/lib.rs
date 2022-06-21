mod game;
mod gui;
mod input_action;
mod network;
// mod reducer;
mod resources;
mod selecting;
mod store;
mod utils;

use game::*;
use gui::*;
use input_action::*;
use network::*;
// use reducer::*;
use resources::*;
use selecting::*;
use store::*;
use utils::*;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
