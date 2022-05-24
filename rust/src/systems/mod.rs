use crate::*;
use gdnative::prelude::*;

//rename?
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
    pub fn get_dragging_id(&self) -> CardId {
        self.dragging.get_dragging_id()
    }
    pub fn drop(&mut self) {
        self.dragging.drop();
    }
    pub fn drop_without_target(&mut self) {
        self.dragging.drop_without_target();
    }
}
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
#[derive(Default)]
pub struct Dragging {
    pub select_card: Option<CardId>,
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
    pub fn drop(&mut self) {
        //card: &mut Card
        self.select_card = None;
        self.cached_pos = None;
        self.drop_back = false;
        // self.select_card = None;
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
