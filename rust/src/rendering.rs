use crate::*;
use gdnative::prelude::*;
// use gdnative::api::{Node, Node2D};
// use player::components::card::Card;
// use gdnative::Ref;
use std::{collections::HashMap, slice::SliceIndex};

pub enum RenderEvent {
    None,
    Highlight,
    // Drag,
}

pub type RenderResponse = (ResponseType, RenderEvent);
pub struct Rendering {
    // cards: HashMap<String, Ref<Node>>,
    events: Vec<RenderResponse>, //needed?
    hovereding: Hoverding,
    dragging: Dragging,
}
impl Rendering {
    // pub fn push_event(&mut self, event: RenderResponse) {
    //     self.events.push(event);
    // }
    pub fn pre_draw(&mut self) {}
    pub fn draw(&mut self, owner: &Node, resource: &mut Resources) {
        while let Some((res, event)) = self.events.pop() {
            match res {
                // ResponseType::TabelCard(card_id) => {
                //     self.draw_card(resource, card_id, event); }
                // ResponseType::HandCard(card_id) => {
                //     self.draw_card(resource, card_id, event);
                // }
                _ => {}
            }
        }
    }
    pub fn after_draw(&mut self, res: &mut Resources, sense: Sense) {
        let card_offset = vec2(0., 30.);
        self.hovereding.run(res, &sense, card_offset);
        self.dragging.run(res, sense.mouse_position(), card_offset);
    }
    pub fn hovered(&mut self, select_id: CardId) {
        self.hovereding.select_id = Some(select_id);
    }
    pub fn drag(&mut self, select_id: CardId) {
        self.dragging.select_card = Some(select_id);
    }
    pub fn is_dragging(&self) -> bool {
        self.dragging.is_dragging()
    }
    pub fn get_dragging_id(&self) -> CardId {
        self.dragging.get_dragging_id()
    }
    // pub fn drop(&mut self) {
    //     self.dragging.drop(res, rendering);
    // }
    pub fn drop_without_target(&mut self) {
        self.dragging.drop_without_target();
    }

    // fn draw_card(&mut self, resource: &mut Resources, card_id: CardId, event: RenderEvent) {
    //     //res: &mut Resources,
    //     match event {
    //         // RenderEvent::None(hash_id) => self.default_draw_card(card),
    //         // RenderEvent::Highlight(hash_id) => self.highlight_draw_card(card),
    //         // RenderEvent::Hovered => self.hovered(card_id),
    //         RenderEvent::Drag => self.drag(resource.get_card(card_id)),
    //         // CardState::Hovered => hovered = card.id, //self.hovered_draw_card(card.texture.clone(), card.x, card.y),
    //         // CardState::Dragging => dragging = card.id, //((x, y)) => self.default_draw_card(card.texture.clone(), x, y),
    //         _ => {} // CardState::NoneDraw => {}
    //     }
    // }
}
impl Default for Rendering {
    fn default() -> Self {
        Self {
            // cards: HashMap::with_capacity(Resources::START_CARD_COUNT),
            events: Vec::with_capacity(6),
            hovereding: Hoverding::new(),
            dragging: Dragging::new(),
        }
    }
}
struct Hoverding {
    select_id: Option<CardId>,
    // cached_id: ,
    cached_id: Option<CardId>, //CardId,
    cached_pos: Vec2,
}
impl Hoverding {
    //fn default
    pub fn new() -> Self {
        Self {
            select_id: None,

            cached_id: None, //CardId::default(),
            cached_pos: Vec2::ZERO,
        }
    }
    pub fn set(&mut self, resources: &mut Resources, select_id: CardId, card_offset: Vec2) {
        let node = unsafe { resources.get_card(select_id).node.assume_safe() };
        self.cached_id = Some(select_id);
        self.cached_pos = node.position();
        node.set_position(self.cached_pos - card_offset, false);
        node.set_scale(vec2(1.5, 1.5));
        // // z-index +1
    }

    fn reset(&mut self, resources: &mut Resources, cached_id: CardId) {
        let node = unsafe { resources.get_card(cached_id).node.assume_safe() };
        node.set_position(self.cached_pos, false);
        node.set_scale(vec2(1., 1.));
        self.cached_id = None; //CardId::default();
        self.cached_pos = Vec2::ZERO;
        // z-index -1
    }
    pub fn run(&mut self, resources: &mut Resources, sense: &Sense, card_offset: Vec2) {
        if let Some(cached_id) = self.cached_id {
            if let Some(select_id) = self.select_id.take() {
                //reset + set
                if select_id != cached_id {
                    // let pos = self.cached_pos;
                    //if !sense.contains_card(pos.x, pos.y) {
                    self.reset(resources, cached_id);
                    self.set(resources, select_id, card_offset);
                }
            } else {
                // reset
                let pos = self.cached_pos;
                if !sense.contains_card(pos.x, pos.y) {
                    self.reset(resources, cached_id);
                }
            }
        } else if let Some(select_id) = self.select_id.take() {
            //set
            self.set(resources, select_id, card_offset);
        }
    }
}
struct Dragging {
    select_card: Option<CardId>,
    cached_pos: Option<Vec2>,
    drop_back: bool,
}
impl Dragging {
    pub fn new() -> Self {
        Self {
            select_card: None,
            cached_pos: None,
            drop_back: false,
        }
    }
    pub fn is_dragging(&self) -> bool {
        self.select_card.is_some()
    }
    pub fn get_dragging_id(&self) -> CardId {
        self.select_card.unwrap()
    }
    pub fn run(&mut self, res: &mut Resources, pos: Vec2, card_offset: Vec2) {
        if let Some(select_id) = self.select_card {
            if self.drop_back {
                if let Some(cached_pos) = self.cached_pos {
                    let node = unsafe { res.get_card(select_id).node.assume_safe() };
                    node.set_position(cached_pos + card_offset, false);
                    self.select_card = None;
                    self.cached_pos = None;
                    self.drop_back = false;
                }
            } else {
                let node = unsafe { res.get_card(select_id).node.assume_safe() };
                if self.cached_pos.is_none() {
                    self.cached_pos = Some(node.position());
                    // node.set_scale(vec2(1.5, 1.5));
                    // // z-index -1
                }
                node.set_position(pos, false);
            }
        }
    }
    pub fn drop(&mut self, card: &mut Card) {
        self.select_card = None;
        // node.set_scale(vec2(1., 1.));
        // // z-index +1

        //if non target to drop
        //else handle
    }
    pub fn drop_without_target(&mut self) {
        self.drop_back = true;
    }
    // pub fn drop(&mut self, res: Response, rendering: &mut Rendering) {
    //     match res.item {
    //         ResponseType::TabelCard(_) | ResponseType::Tabel => {
    //             //cast to tabel
    //             // let card_cost = rendering.get_card_cost(fit_card_id);
    //             //                     if self.side_client.player.try_pay_mana(card_cost) {
    //             //                         self.queue_command.push(
    //             //                             CommandBuilder::default().line(LineType::Hand).build(
    //             //                                 PlayerType::Client,
    //             //                                 Event::cast_on_tabel(fit_card_id),
    //             //                             ),
    //             //                         );
    //             //                         rendering.drop();
    //             //                     }

    //             // LineType::Tabel => {
    //             //                 let card_cost = rendering.get_card_cost(fit_card_id);
    //             //                 if self.side_client.player.try_pay_mana(card_cost) {
    //             //                     self.queue_command.push(
    //             //                         CommandBuilder::default().line(LineType::Hand).build(
    //             //                             PlayerType::Client,
    //             //                             Event::cast_on_tabel(fit_card_id),
    //             //                         ),
    //             //                     );
    //             //                     rendering.drop();
    //             //                 }
    //             //             }
    //             self.select_card = None;
    //             let node = unsafe { card.node.assume_safe() };
    //             // node.set_scale(vec2(1., 1.));
    //             // // z-index +1

    //             //if non target to drop
    //         }
    //         ResponseType::HandCard(card_id) => {
    //             match res.player {
    //                 PlayerType::Client => {
    //                     // swap card
    //                     // self.player_client
    //                     //     .swap_card_on_hand(dragging_card_id, card_id);
    //                 }
    //                 PlayerType::Remote => {}
    //             }
    //         }
    //         ResponseType::Hand => {}
    //         _ => {
    //             //drop
    //             if let Some(pos) = self.cached_pos {
    //                 let node = unsafe { card.node.assume_safe() };
    //                 node.set_position(pos, false);
    //             }
    //         }
    //     }
    // }
}
