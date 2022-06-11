use crate::*;

#[derive(Default)]
pub struct Dragging {
    pub select_card: Option<CardId>,
}
impl Dragging {
    pub fn is_some(&self) -> bool {
        self.select_card.is_some()
    }
    // pub fn get_id(&mut self) -> CardId {
    //     //??
    //     // self.cached_pos = None;
    //     // self.drop_back = false;
    //     self.select_card.unwrap()
    // }
    pub fn run(&mut self, ctx: &mut Resources, pos: Vec2, card_offset: Vec2) {
        if let Some(select_id) = self.select_card {
            // if self.drop_back {
            //     if let Some(cached_pos) = self.cached_pos {
            //         let node = unsafe { res.get_card(select_id).node.assume_safe() };
            //         // node.set_global_position(cached_pos + card_offset, false);
            //         node.set_global_position(cached_pos, false);
            //         self.select_card = None;
            //         self.cached_pos = None;
            //         self.drop_back = false;
            //     }
            // } else {
            //     let node = unsafe { res.get_card(select_id).node.assume_safe() };
            //     if self.cached_pos.is_none() {
            //         self.cached_pos = Some(node.global_position());
            //         // node.set_scale(vec2(1.5, 1.5));
            //         // // z-index -1
            //     }
            //     node.set_global_position(pos, false);
            // }
            let node = unsafe { ctx.get_card(select_id).node.assume_safe() };
            node.set_global_position(pos, false);
        }
    }
    pub fn drop(&mut self) {
        //card: &mut Card
        self.select_card = None;
        // self.cached_pos = None;
        // self.drop_back = false;
        // self.select_card = None;
        // node.set_scale(vec2(1., 1.));
        // // z-index +1

        //if non target to drop
        //else handle
    }
    pub fn drop_without_target(&mut self) {
        // self.drop_back = true;
        self.select_card = None;
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
