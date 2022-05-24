use gdnative::prelude::godot_print;

use super::line::*;
use crate::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Tabel {
    pub line: Line,
}
impl Tabel {
    pub fn new(rect: Rect, capacity: usize, card_size: Vec2) -> Self {
        godot_print!("!!!!!Rect Tabel:{}", rect);
        Self {
            line: Line {
                rect,
                draw_query: VecDeque::with_capacity(capacity),
                indent: vec2(10., 0.),
                event_pos_changed: false,
                card_size,
            },
        }
    }
    pub fn contains(&self, sense: &Sense) -> bool {
        self.line.contains(sense)
    }
    pub fn input_handler(&self, sense: Sense) -> ResponseType {
        if let Some(id) = self.line.input_handler(sense) {
            return ResponseType::TabelCard(id);
        }
        ResponseType::Tabel
        // let (mut x, y) = self.lining(sense.card_size, true); //from reversestart alight line
        // x -= sense.card_size.x + self.indent.x;
        // for id in self.draw_query.iter().rev() {
        //     if sense.contains(x, y) {
        //         sense.line(self.line_type).item(ResponseType::Card(*id));
        //         return;
        //     }
        //     x -= sense.card_size.x + self.indent.x;
        // }
        // sense.line(self.line_type).item(ResponseType::Line);
    }
    pub fn add_card(&mut self, card_id: CardId) {
        self.line.front_add_card(card_id);
    }
    pub fn remove_card(&mut self, card_id: CardId) {
        self.line.remove_card(card_id);
    }
    // pub fn create_card(&mut self, name: &'static str) {
    //     self.front_add_card(storage::get_mut::<Resources>().new_card(name));
    // }

    // pub fn front_add_card(&mut self, card_id: u64) {
    //     self.event_pos_changed = true;
    //     self.draw_query.push_front(card_id);
    // }
    // pub fn back_add_card(&mut self, card_id: u64) {
    //     self.event_pos_changed = true;
    //     self.draw_query.push_back(card_id);
    // }
    // pub fn remove_card(&mut self, card_id: u64) {
    //     self.event_pos_changed = true;
    //     let i = self
    //         .find_id(card_id)
    //         .expect("Card dont can remove? not found");
    //     self.draw_query.remove(i);
    // }
    // pub fn swap_card(&mut self, origin_draw_id: u64, target_card_id: u64) {
    //     self.event_pos_changed = true;
    //     if let Some(target_draw_id) = self.find_id(target_card_id) {
    //         if let Some(origin_draw_id) = self.find_id(origin_draw_id) {
    //             self.draw_query.swap(origin_draw_id, target_draw_id);
    //         }
    //     }
    // }
    // // pub fn query_hand(&mut self, ctx: &mut Rendering, cost: u64) {
    // //     for id in self.draw_query.iter() {
    // //         ctx.query_hand(*id, cost);
    // //     }
    // // }
    // // pub fn set_position(&mut self, ctx: &mut Rendering) {
    // //     if self.event_pos_changed {
    // //         let (mut x, y) = self.lining(card_size, false); //from start alight line
    // //         for id in self.draw_query.iter() {
    // //             ctx.set_card_pos(*id, x, y);
    // //             x += card_size.x + self.indent.x;
    // //         }
    // //         self.event_pos_changed = false;
    // //     }
    // // }
    // pub fn contains(&self, sense: &mut Sense) -> bool {
    //     let pos = self.rect.point();
    //     let size = self.rect.size();
    //     sense.contains(pos.x, pos.y)
    // }

    // pub fn find_id(&mut self, id: u64) -> Option<usize> {
    //     let y = self.draw_query.iter().position(|&x| x == id);
    //     y
    // }
    pub fn set_position(&mut self, res: &mut Resources) {
        self.line.set_position(res);
    }
}
