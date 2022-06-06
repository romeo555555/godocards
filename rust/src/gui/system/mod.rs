use crate::*;
use gdnative::prelude::*;
mod dragging;
pub use dragging::*;
mod hovered;
pub use hovered::*;

pub enum LineType {
    None,
    Tabel(PlayerId),
    Hand(PlayerId),
}
pub struct SelectCard {
    hovereding: Hoverding,
    dragging: Dragging,
    client_id: PlayerId,
    event: Vec<Message>,
}
impl SelectCard {
    pub fn new(client_id: PlayerId) -> Self {
        Self {
            hovereding: Hoverding::default(),
            dragging: Dragging::default(),
            client_id,
            event: Vec::with_capacity(5),
        }
    }
    pub fn send_msg(&mut self, event: Event) {
        godot_print!("send event : {:?}", event);
        self.event.push(Message::build(self.client_id, event));
    }
    pub fn pop_event(&mut self) -> Option<Message> {
        self.event.pop()
    }
    pub fn run(&mut self, ctx: &mut Resources, sense: Sense) {
        let card_offset = vec2(0., 30.);
        self.hovereding.run(ctx, &sense, card_offset);
        self.dragging.run(ctx, sense.mouse_position(), card_offset);
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
    pub fn get_dragging_id(&mut self) -> CardId {
        self.dragging.get_dragging_id()
    }
    pub fn drop(&mut self) {
        self.dragging.drop();
    }
    pub fn drop_without_target(&mut self) {
        self.dragging.drop_without_target();
    }
}
