pub mod card;
mod line;
pub mod player;

use crate::*;
use card::*;
use gdnative::api::{Label, Node, Node2D, PackedScene, ResourceLoader, TextureRect};
use gdnative::object::{Ref, TRef};
use gdnative::prelude::Shared;
use gdnative::{api::Texture, prelude::godot_print};
use line::*;
use nanoserde::{DeBin, SerBin};
pub use player::*;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::{cmp::Ordering, ops::Add};

// pub enum RenderEvent {
//     Hovered(CardId),
//     Drag(CardId),
//     Drop(CardId),
//     SortPosLine,
// }
// pub struct Gui {
//     hovereding: Hoverding,
//     dragging: Dragging,
//     sorting: Option<LineType>,
//     //render_qeu
//     // render_event: Vec<RenderEvent>,
// }
// impl Gui {
//     // pub fn draw(&mut self, ctx: &mut Resources, sense: Sense) {
//     //     if let Some(event) = self.render_event.pop() {
//     //         match event {
//     //             RenderEvent::Hovered(card_id) => {
//     //                 self.hovereding.select_id = Some(card_id);
//     //             }
//     //             RenderEvent::Drag(card_id) => {
//     //                 self.dragging.select_card = Some(card_id);
//     //             }
//     //             RenderEvent::Drop(card_id) => {}
//     //             RenderEvent::SortPosLine => {}
//     //         }
//     //     }
//     //     self.run(ctx, sense);
//     // }
//     pub fn run(&mut self, ctx: &mut Resources, sense: Sense) {
//         let card_offset = vec2(0., 30.);
//         self.hovereding.run(ctx, &sense, card_offset);
//         self.dragging.run(ctx, sense.mouse_position(), card_offset);
//     }

//     pub fn is_dragging(&self) -> bool {
//         self.dragging.is_dragging()
//     }
//     pub fn get_dragging_id(&mut self) -> CardId {
//         self.dragging.get_dragging_id()
//     }
//     pub fn drop(&mut self) {
//         self.dragging.drop();
//     }
//     pub fn drop_without_target(&mut self) {
//         self.dragging.drop_without_target();
//     }
// }
