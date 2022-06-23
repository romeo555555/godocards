mod game;
mod gui;
mod input;
mod layout;
mod network;
mod resources;
mod store;
mod utils;

use game::*;
use gui::*;
use input::*;
use layout::*;
use network::*;
use resources::*;
use store::*;
use utils::*;

use gdnative::prelude::{godot_init, InitHandle, Input as GodoInput};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
