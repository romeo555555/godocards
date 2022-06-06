use crate::*;
// extern crate conv;
use conv::*;
use gdnative::prelude::godot_print;
use std::collections::{HashMap, HashSet, VecDeque};
pub struct Line {
    pub rect: Rect,
    pub indent: Vec2,
    pub draw_query: VecDeque<CardId>,
    pub event_pos_changed: bool,
    pub card_size: Vec2,
}
impl Line {
    pub fn new(rect: Rect, capacity: usize, card_size: Vec2) -> Self {
        // godot_print!("!!!!!Rect Hand:{}", rect);
        Self {
            rect,
            draw_query: VecDeque::with_capacity(capacity),
            indent: vec2(10., 0.),
            event_pos_changed: false,
            card_size,
        }
    }
    // pub fn create_card(&mut self, name: &'static str) {
    //     self.front_add_card(storage::get_mut::<Resources>().new_card(name));
    // }
    pub fn add_card(&mut self, card_id: CardId) {
        self.front_add_card(card_id);
    }
    pub fn front_add_card(&mut self, card_id: CardId) {
        self.event_pos_changed = true;
        self.draw_query.push_front(card_id);
    }
    pub fn back_add_card(&mut self, card_id: CardId) {
        self.event_pos_changed = true;
        self.draw_query.push_back(card_id);
    }
    pub fn remove_card(&mut self, card_id: CardId) {
        self.event_pos_changed = true;
        let i = self
            .find_id(card_id)
            .expect("Card dont can remove? not found");
        self.draw_query.remove(i);
    }
    pub fn swap_card(&mut self, origin_draw_id: CardId, target_card_id: CardId) {
        self.event_pos_changed = true;
        if let Some(target_draw_id) = self.find_id(target_card_id) {
            if let Some(origin_draw_id) = self.find_id(origin_draw_id) {
                self.draw_query.swap(origin_draw_id, target_draw_id);
            }
        }
    }
    // pub fn query_hand(&mut self, ctx: &mut Rendering, cost: u64) {
    //     for id in self.draw_query.iter() {
    //         ctx.query_hand(*id, cost);
    //     }
    // }
    // pub fn input_handler(&self, sense: Sense) -> ResponseType {
    //        if let Some(id) = self.line.input_handler(sense) {
    //            return ResponseType::HandCard(id);
    //        }
    //        ResponseType::Hand
    //    }
    //  pub fn swap_card(&mut self, origin_draw_id: u64, target_card_id: u64) {
    //      self.line.event_pos_changed = true;
    //      if let Some(target_draw_id) = self.line.find_id(target_card_id) {
    //          if let Some(origin_draw_id) = self.line.find_id(origin_draw_id) {
    //              self.line.draw_query.swap(origin_draw_id, target_draw_id);
    //          }
    //      }
    //  }
    pub fn contains(&self, sense: &Sense) -> bool {
        sense.contains_rect(&self.rect)
    }
    pub fn input_handler(&self, sense: Sense) -> Option<CardId> {
        let (mut x, y) = self.line_start_point(sense.card_size);
        let x_indent = self.card_size.x + self.indent.x;
        for card_id in self.draw_query.iter() {
            if sense.contains_card(x, y) {
                return Some(*card_id);
            }

            // godot_print!(
            //     "card input @:{} pos: {}-{},,, card_size x:{}, y;{}",
            //     card_id,
            //     x,
            //     y,
            //     sense.card_size.x,
            //     sense.card_size.y,
            // );
            x += x_indent;
        }
        None
    }
    pub fn set_position(&mut self, res: &mut Resources) {
        if self.event_pos_changed {
            let (mut x, y) = self.line_start_point(self.card_size);
            let x_indent = self.card_size.x + self.indent.x;
            for card_id in self.draw_query.iter() {
                res.set_card_pos(*card_id, vec2(x, y));
                godot_print!(
                    "card set_pos @:{} pos: {}-{},,, card_size x:{}, y;{}",
                    card_id,
                    x,
                    y,
                    self.card_size.x,
                    self.card_size.y,
                );
                x += x_indent;
            }

            // godot_print!("!!!!!Rect hand:{}", self.rect);
            self.event_pos_changed = false;
        }
    }
    fn line_start_point(&self, card_size: Vec2) -> (f32, f32) {
        let count_cards = f32::value_from(self.draw_query.len()).unwrap();
        (
            if count_cards == 0. {
                self.rect.center_x - (card_size.x / 2.)
            } else {
                self.rect.center_x
                    - ((count_cards * card_size.x + (count_cards - 1.) * self.indent.x) / 2.)
            },
            self.rect.center_y - (card_size.y / 2.),
        )
        //if count_cards > max_count on 0ne line / 2. line
    }
    pub fn find_id(&mut self, id: CardId) -> Option<usize> {
        let y = self.draw_query.iter().position(|&x| x == id);
        y
    }
}
// #[derive(Copy, Clone, Debug)]
// pub enum LineType {
//     None,
//     Hand,
//     Tabel,
// }
// impl Default for LineType {
//     fn default() -> Self {
//         LineType::None
//     }
// }
