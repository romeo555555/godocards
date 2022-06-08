use crate::*;

#[derive(Default)]
pub struct Hoverding {
    pub select_card: Option<CardId>,
    cached_card: Option<CardId>, //CardId,
    cached_pos: Vec2,
}
impl Hoverding {
    pub fn set(&mut self, resources: &mut Resources, select_card: CardId, card_offset: Vec2) {
        let node = unsafe { resources.get_card(select_card).node.assume_safe() };
        self.cached_card = Some(select_card);
        self.cached_pos = node.global_position();
        //dont global?
        node.set_global_position(self.cached_pos - card_offset, false);
        node.set_scale(vec2(1.5, 1.5));
        // // z-index +1
    }

    fn reset(&mut self, resources: &mut Resources, cached_card: CardId) {
        let node = unsafe { resources.get_card(cached_card).node.assume_safe() };
        node.set_global_position(self.cached_pos, false);
        node.set_scale(vec2(1., 1.));
        self.cached_card = None; //CardId::default();
        self.cached_pos = Vec2::ZERO;
        // z-index -1
    }
    pub fn run(&mut self, resources: &mut Resources, sense: &Sense, card_offset: Vec2) {
        if let Some(cached_card) = self.cached_card {
            if let Some(select_card) = self.select_card.take() {
                //reset + set
                if select_card != cached_card {
                    // let pos = self.cached_pos;
                    //if !sense.contains_card(pos.x, pos.y) {
                    self.reset(resources, cached_card);
                    self.set(resources, select_card, card_offset);
                }
            } else {
                // reset
                let pos = self.cached_pos;
                if !sense.contains_card(pos.x, pos.y) {
                    self.reset(resources, cached_card);
                }
            }
        } else if let Some(select_card) = self.select_card.take() {
            //set
            self.set(resources, select_card, card_offset);
        }
    }
}
