mod game;
mod gui;
mod input;
mod matchmaking;
mod network;
mod resources;
mod utils;

use crate::gui::card::*;
use game::*;
use gui::*;
use input::*;
use matchmaking::*;
use network::*;
use resources::*;
use utils::*;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<game::Game>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
