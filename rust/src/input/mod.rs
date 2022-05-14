use crate::*;

#[derive(Clone, Copy)]
pub struct Sense {
    pub card_size: Vec2,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub click_up: bool,
    pub click_down: bool,
}
impl Sense {
    pub fn new(mp: Vec2, card_size: Vec2, click_up: bool, click_down: bool) -> Self {
        // off if windiw game not focused
        // let (mouse_x, mouse_y) = (0., 0.); //mouse_position();
        Self {
            card_size,
            mouse_x: mp.x,
            mouse_y: mp.y,
            click_up,
            click_down,
        }
    }
    pub fn contains_rect(&self, rect: &Rect) -> bool {
        self.mouse_x >= rect.left()
            && self.mouse_x < rect.right()
            && self.mouse_y < rect.bottom()
            && self.mouse_y >= rect.top()
    }
    pub fn contains_card(&self, x: f32, y: f32) -> bool {
        self.mouse_x >= x
            && self.mouse_x < x + self.card_size.x
            && self.mouse_y < y + self.card_size.y
            && self.mouse_y >= y
    }
    pub fn mouse_position(&self) -> Vec2 {
        vec2(self.mouse_x, self.mouse_y)
    }
}
#[derive(Debug)]
pub struct Response {
    pub item: ResponseType,
    pub click_down: bool,
    pub click_up: bool,
    pub player_id: PlayerId,
}
impl Response {
    pub fn new(
        item: ResponseType,
        player_id: PlayerId,
        click_up: bool,
        click_down: bool,
    ) -> Response {
        //build_response
        Response {
            item,
            click_down,
            click_up,
            player_id,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ResponseType {
    None,
    TabelCard(CardId),
    HandCard(CardId),
    Tabel,
    Hand,
    Deck,
    Builds,
    Items,
    Character,
    //Avatar
}
