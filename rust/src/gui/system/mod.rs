use crate::*;
use gdnative::prelude::*;
mod dragging;
pub use dragging::*;
mod hovered;
pub use hovered::*;

enum SelectedState {
    None,
    Hoverd(CardId),
    Dragging(CardId),
}
pub struct SelectingCard {
    select_card: SelectedState,
    hovereding: Hoverding,
    dragging: Dragging,
    // client_id: PlayerId,
}
impl SelectingCard {
    pub fn new() -> Self {
        Self {
            select_card: SelectedState::None,
            hovereding: Hoverding::default(),
            dragging: Dragging::default(),
            // client_id,
        }
    }
    pub fn hovered(&mut self, select_id: CardId) {
        self.select_card = SelectedState::Hoverd(select_id);
    }
    pub fn drag(&mut self, select_id: CardId) {
        self.select_card = SelectedState::Dragging(select_id);
    }
    pub fn drop(&mut self) {
        self.select_card = SelectedState::None;
    }
    pub fn is_dragging(&self) -> bool {
        if let SelectedState::Dragging(_) = self.select_card {
            return true;
        }
        false
    }
    pub fn get_dragging_id(&mut self) -> Option<CardId> {
        if let SelectedState::Dragging(card_id) = self.select_card {
            return Some(card_id);
        }
        None
    }
    // pub fn drop_without_target(&mut self) {
    //     self.dragging.drop_without_target();
    // }
    pub fn run(&mut self, ctx: &mut Resources, sense: Sense) {
        let card_offset = vec2(0., 30.);
        self.hovereding.run(ctx, &sense, card_offset);
        self.dragging.run(ctx, sense.mouse_position(), card_offset);
    }
}
