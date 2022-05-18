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
    // pub fn pre_draw(&mut self) {}
    // pub fn draw(&mut self, owner: &Node, resource: &mut Resources) {
    //     while let Some((res, event)) = self.events.pop() {
    //         match res {
    //             // ResponseType::TabelCard(card_id) => {
    //             //     self.draw_card(resource, card_id, event); }
    //             // ResponseType::HandCard(card_id) => {
    //             //     self.draw_card(resource, card_id, event);
    //             // }
    //             _ => {}
    //         }
    //     }
    // }
    pub fn after_draw(&mut self, resources: &mut Resources, sense: Sense) {
        let card_offset = vec2(0., 30.);
        self.hovereding.run(resources, &sense, card_offset);
        self.dragging
            .run(resources, sense.mouse_position(), card_offset);
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
