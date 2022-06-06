use crate::*;

#[derive(Default)]
pub struct Hoverding {
    pub select_id: Option<CardId>,
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
        self.cached_pos = node.global_position();
        //dont global?
        node.set_global_position(self.cached_pos - card_offset, false);
        node.set_scale(vec2(1.5, 1.5));
        // // z-index +1
    }

    fn reset(&mut self, resources: &mut Resources, cached_id: CardId) {
        let node = unsafe { resources.get_card(cached_id).node.assume_safe() };
        node.set_global_position(self.cached_pos, false);
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
