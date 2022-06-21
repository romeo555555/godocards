mod component;
mod game;
mod input;
mod network;
mod resources;
mod system;
mod utils;

use component::*;
use game::*;
use input::*;
use network::*;
use resources::*;
use system::*;
use utils::*;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
