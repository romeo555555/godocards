use crate::*;
// extern crate conv;
// use conv::*;
use gdnative::prelude::godot_print;
use std::collections::{HashMap, HashSet, VecDeque};
pub struct Line {
    rect: Rect,
    cards: VecDeque<CardId>, //maybe slice [CardId] ?
    count: f32,
    exclude_card: Option<CardId>,
    pub indent: Vec2,
    // pub event_pos_changed: bool,
}
impl Line {
    pub fn new(rect: Rect, capacity: usize) -> Self {
        // godot_print!("!!!!!Rect Hand:{}", rect);
        Self {
            rect,
            cards: VecDeque::with_capacity(capacity),
            indent: vec2(10., 0.),
            count: 0.,
            exclude_card: None,
            // event_pos_changed: false,
        }
    }
    // pub fn front_add_card(&mut self, card_id: CardId) {
    //     // self.event_pos_changed = true;
    //     self.cards.push_front(card_id);
    // }
    // pub fn back_add_card(&mut self, card_id: CardId) {
    //     // self.event_pos_changed = true;
    //     self.cards.push_back(card_id);
    // }
    pub fn add_card(&mut self, card_id: CardId) {
        self.count += 1.;
        self.cards.push_front(card_id);
    }
    pub fn remove_card(&mut self, card_id: CardId) {
        // self.event_pos_changed = true;
        self.count -= 1.;
        let i = self
            .cards
            .iter()
            .position(|&x| x == card_id)
            .expect("Card dont can remove? not found");
        self.cards.remove(i);
    }
    pub fn swap_card(&mut self, origin_draw_id: CardId, target_card_id: CardId) {
        // self.event_pos_changed = true;
        if let Some(target_draw_id) = self.cards.iter().position(|&x| x == target_card_id) {
            if let Some(origin_draw_id) = self.cards.iter().position(|&x| x == origin_draw_id) {
                self.cards.swap(origin_draw_id, target_draw_id);
            }
        }
    }
    pub fn contains(&self, sense: &Sense) -> bool {
        sense.contains_rect(&self.rect)
    }
    pub fn input_handler(&self, sense: Sense, card_size: Vec2) -> Option<CardId> {
        if let Some((mut x, y)) = self.alignment_start_point(sense.card_size, false) {
            let x_indent = card_size.x + self.indent.x;
            let iter = self.cards.iter();
            for card_id in iter {
                if sense.contains_card(x, y) {
                    return Some(*card_id);
                }
                //         // godot_print!(
                //         //     "card input @:{} pos: {}-{},,, card_size x:{}, y;{}",
                //         //     card_id,
                //         //     x,
                //         //     y,
                //         //     sense.card_size.x,
                //         //     sense.card_size.y,
                //         // );
                x += x_indent;
            }
        }
        None
    }
    pub fn input_handler_witch_exclude(
        &self,
        sense: Sense,
        card_size: Vec2,
        exclude_card: CardId,
    ) -> Option<CardId> {
        if let Some((mut x, y)) = self.alignment_start_point(sense.card_size, true) {
            let x_indent = card_size.x + self.indent.x;
            let mut iter = self.cards.iter();
            for _ in 0..self.cards.len() {
                let card_id = iter.next().unwrap();
                if sense.contains_card(x, y) {
                    if *card_id == exclude_card {
                        return Some(*iter.next().unwrap());
                    }
                    return Some(*card_id);
                }
                x += x_indent;
            }
        }
        None
    }

    // pub fn set_position(&mut self, res: &mut Resources) {
    //     if self.event_pos_changed {
    //         let (mut x, y) = self.line_start_point(self.card_size);
    //         let x_indent = self.card_size.x + self.indent.x;
    //         for card_id in self.cards.iter() {
    //             res.set_card_pos(*card_id, vec2(x, y));
    //             godot_print!(
    //                 "card set_pos @:{} pos: {}-{},,, card_size x:{}, y;{}",
    //                 card_id,
    //                 x,
    //                 y,
    //                 self.card_size.x,
    //                 self.card_size.y,
    //             );
    //             x += x_indent;
    //         }

    //         // godot_print!("!!!!!Rect hand:{}", self.rect);
    //         self.event_pos_changed = false;
    //     }
    // }
    pub fn alignment_start_point(&self, card_size: Vec2, has_exclude: bool) -> Option<(f32, f32)> {
        let count_cards = if has_exclude {
            self.count - 1.
        } else {
            self.count
        };
        return if count_cards < 1. {
            None
        } else if count_cards == 1. {
            Some((
                self.rect.center_x - (card_size.x / 2.),
                self.rect.center_y - (card_size.y / 2.),
            ))
        } else {
            Some((
                self.rect.center_x
                    - ((count_cards * card_size.x + (count_cards - 1.) * self.indent.x) / 2.),
                self.rect.center_y - (card_size.y / 2.),
            ))
            //if count_cards > max_count on 0ne line / 2. line
        };
    }
    pub fn sort_line(
        &mut self,
        ctx: &mut Resources,
        line_type: LineType,
        exclude_card: Option<CardId>,
    ) {
        if let Some(exclude_card) = self.exclude_card {
            let card_size = ctx.card_size();
            if let Some((mut x, y)) = self.alignment_start_point(card_size, true) {
                let x_indent = card_size.x + ctx.card_indent().x;

                let mut iter = self.cards.iter();
                for _ in 0..self.cards.len() {
                    let card_id = iter
                        .next()
                        .map(|card_id| {
                            if *card_id == exclude_card {
                                return *iter.next().unwrap();
                            }
                            *card_id
                        })
                        .unwrap();
                    ctx.set_card_pos(card_id, vec2(x, y));
                    godot_print!(
                        "card set_pos @:{} pos: {}-{},,, card_size x:{}, y;{}",
                        card_id,
                        x,
                        y,
                        card_size.x,
                        card_size.y,
                    );
                    x += x_indent;
                }
            }
        } else {
            let card_size = ctx.card_size();
            if let Some((mut x, y)) = self.alignment_start_point(card_size, false) {
                let x_indent = card_size.x + ctx.card_indent().x;
                for card_id in self.cards.iter() {
                    ctx.set_card_pos(*card_id, vec2(x, y));
                    godot_print!(
                        "card set_pos @:{} pos: {}-{},,, card_size x:{}, y;{}",
                        card_id,
                        x,
                        y,
                        card_size.x,
                        card_size.y,
                    );
                    x += x_indent;
                }
            }
        }
    }

    // pub fn query_hand(&mut self, ctx: &mut Rendering, cost: u64) {
    //     for id in self.draw_query.iter() {
    //         ctx.query_hand(*id, cost);
    //     }
    // }
    //  pub fn swap_card(&mut self, origin_draw_id: u64, target_card_id: u64) {
    //      self.line.event_pos_changed = true;
    //      if let Some(target_draw_id) = self.line.find_id(target_card_id) {
    //          if let Some(origin_draw_id) = self.line.find_id(origin_draw_id) {
    //              self.line.draw_query.swap(origin_draw_id, target_draw_id);
    //          }
    //      }
    //  }
}
