// use crate::*;
// use gdnative::prelude::*;
// mod dragging;
// pub use dragging::*;
// mod hovered;
// pub use hovered::*;
use common::{
    card::{CardId, CardState},
    game_match::MatchInfo,
    mana::{Mana, ManaColor, ManaForm},
    player::{PlayerData, PlayerId, PlayerState},
};

use crate::{
    gui::Gui,
    input_action::Sense,
    resources::Resources,
    utils::{contains_card, vec2, Vec2},
};

#[derive(Clone, Copy)]
pub enum SelectedState {
    None,
    Hoverd(CardId),
    Dragging(CardId),
}
pub struct SelectingCard {
    select_card: SelectedState,
    // hovereding: Hoverding,
    // dragging: Dragging,
    // client_id: PlayerId,
    cached_hovered_card: Option<(CardId, Vec2)>,
    // cached_pos: Vec2,
}
impl SelectingCard {
    pub fn new() -> Self {
        Self {
            select_card: SelectedState::None,
            // hovereding: Hoverding::default(),
            // dragging: Dragging::default(),
            // client_id,
            cached_hovered_card: None,
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
    pub fn get_id_if_dragging(&mut self) -> Option<CardId> {
        if let SelectedState::Dragging(card_id) = self.select_card {
            return Some(card_id);
        }
        None
    }
    // pub fn drop_without_target(&mut self) {
    //     self.dragging.drop_without_target();
    // }
    pub fn get_state(&mut self) -> SelectedState {
        self.select_card
    }
    pub fn cached_hovered(&self) -> &Option<(CardId, Vec2)> {
        &self.cached_hovered_card
    }
    pub fn set_cached_hovered(&mut self, card_id: CardId, cached_pos: Vec2) {
        self.cached_hovered_card = Some((card_id, cached_pos));
    }
    pub fn cached_hovered_clean(&mut self) {
        self.cached_hovered_card = None;
    }
    pub fn update_selected(&mut self, mouse_pos: Vec2, card_size: Vec2, gui: &mut Gui) {
        let card_offset = vec2(0., 30.);
        match self.get_state() {
            SelectedState::Dragging(card_id) => {
                if let Some((cached_card, pos)) = self.cached_hovered() {
                    // reset
                    if !contains_card(mouse_pos, card_size, pos.x, pos.y) {
                        gui.get_mut_card(cached_card).hovered_off(*pos);
                        self.cached_hovered_clean();
                    }
                }
                //dragging
                gui.get_mut_card(&card_id).set_position(mouse_pos);
            }
            SelectedState::Hoverd(ref card_id) => {
                if let Some((cached_card, pos)) = self.cached_hovered() {
                    //reset + set
                    if card_id != cached_card {
                        gui.get_mut_card(cached_card).hovered_off(card_offset);
                        gui.get_mut_card(card_id).hovered_on(mouse_pos);
                        self.set_cached_hovered(*card_id, mouse_pos);
                    }
                } else {
                    //set
                    gui.get_mut_card(card_id).hovered_on(mouse_pos);
                    self.set_cached_hovered(*card_id, mouse_pos);
                }
            }
            SelectedState::None => {
                if let Some((cached_card, pos)) = self.cached_hovered() {
                    // reset
                    if !contains_card(mouse_pos, card_size, pos.x, pos.y) {
                        gui.get_mut_card(cached_card).hovered_off(*pos);
                        self.cached_hovered_clean();
                    }
                }
            }
        }
    }
}

// #[derive(Default)]
// pub struct Hoverding {
//     // pub select_card: Option<CardId>,
//     cached_card: Option<CardId>, //CardId,
//     cached_pos: Vec2,
// }
// impl Hoverding {
//     pub fn set(&mut self, resources: &mut Resources, select_card: CardId, card_offset: Vec2) {
//         let node = unsafe { resources.get_card(select_card).node.assume_safe() };
//         self.cached_card = Some(select_card);
//         self.cached_pos = node.global_position();
//         //dont global?
//         node.set_global_position(self.cached_pos - card_offset, false);
//         node.set_scale(vec2(1.5, 1.5));
//         // // z-index +1
//     }

//     fn reset(&mut self, resources: &mut Resources, cached_card: CardId) {
//         let node = unsafe { resources.get_card(cached_card).node.assume_safe() };
//         node.set_global_position(self.cached_pos, false);
//         node.set_scale(vec2(1., 1.));
//         self.cached_card = None; //CardId::default();
//         self.cached_pos = Vec2::ZERO;
//         // z-index -1
//     }
//     pub fn run(&mut self, resources: &mut Resources, sense: &Sense, card_offset: Vec2) {
//         if let Some(select_card) = self.select_card.take() {
//             if let Some(cached_card) = self.cached_card {
//                 //reset + set
//                 if select_card != cached_card {
//                     // let pos = self.cached_pos;
//                     //if !sense.contains_card(pos.x, pos.y) {
//                     self.reset(resources, cached_card);
//                     self.set(resources, select_card, card_offset);
//                 }
//             } else {
//                 //set
//                 self.set(resources, select_card, card_offset);
//             }
//         } else {
//             // reset
//             let pos = self.cached_pos;
//             if !sense.contains_card(pos.x, pos.y) {
//                 self.reset(resources, cached_card);
//             }
//         }
//     }
// }
// #[derive(Default)]
// pub struct Dragging {
//     pub select_card: Option<CardId>,
// }
// impl Dragging {
//     pub fn is_some(&self) -> bool {
//         self.select_card.is_some()
//     }
//     // pub fn get_id(&mut self) -> CardId {
//     //     //??
//     //     // self.cached_pos = None;
//     //     // self.drop_back = false;
//     //     self.select_card.unwrap()
//     // }
//     pub fn run(&mut self, ctx: &mut Resources, pos: Vec2, card_offset: Vec2) {
//         if let Some(select_id) = self.select_card {
//             // if self.drop_back {
//             //     if let Some(cached_pos) = self.cached_pos {
//             //         let node = unsafe { res.get_card(select_id).node.assume_safe() };
//             //         // node.set_global_position(cached_pos + card_offset, false);
//             //         node.set_global_position(cached_pos, false);
//             //         self.select_card = None;
//             //         self.cached_pos = None;
//             //         self.drop_back = false;
//             //     }
//             // } else {
//             //     let node = unsafe { res.get_card(select_id).node.assume_safe() };
//             //     if self.cached_pos.is_none() {
//             //         self.cached_pos = Some(node.global_position());
//             //         // node.set_scale(vec2(1.5, 1.5));
//             //         // // z-index -1
//             //     }
//             //     node.set_global_position(pos, false);
//             // }
//             let node = unsafe { ctx.get_card(select_id).node.assume_safe() };
//             node.set_global_position(pos, false);
//         }
//     }
//     pub fn drop(&mut self) {
//         //card: &mut Card
//         self.select_card = None;
//         // self.cached_pos = None;
//         // self.drop_back = false;
//         // self.select_card = None;
//         // node.set_scale(vec2(1., 1.));
//         // // z-index +1

//         //if non target to drop
//         //else handle
//     }
//     pub fn drop_without_target(&mut self) {
//         // self.drop_back = true;
//         self.select_card = None;
//     }
//     // pub fn drop(&mut self, res: Response, rendering: &mut Rendering) {
//     //     match res.item {
//     //         ResponseType::TabelCard(_) | ResponseType::Tabel => {
//     //             //cast to tabel
//     //             // let card_cost = rendering.get_card_cost(fit_card_id);
//     //             //                     if self.side_client.player.try_pay_mana(card_cost) {
//     //             //                         self.queue_command.push(
//     //             //                             CommandBuilder::default().line(LineType::Hand).build(
//     //             //                                 PlayerType::Client,
//     //             //                                 Event::cast_on_tabel(fit_card_id),
//     //             //                             ),
//     //             //                         );
//     //             //                         rendering.drop();
//     //             //                     }

//     //             // LineType::Tabel => {
//     //             //                 let card_cost = rendering.get_card_cost(fit_card_id);
//     //             //                 if self.side_client.player.try_pay_mana(card_cost) {
//     //             //                     self.queue_command.push(
//     //             //                         CommandBuilder::default().line(LineType::Hand).build(
//     //             //                             PlayerType::Client,
//     //             //                             Event::cast_on_tabel(fit_card_id),
//     //             //                         ),
//     //             //                     );
//     //             //                     rendering.drop();
//     //             //                 }
//     //             //             }
//     //             self.select_card = None;
//     //             let node = unsafe { card.node.assume_safe() };
//     //             // node.set_scale(vec2(1., 1.));
//     //             // // z-index +1

//     //             //if non target to drop
//     //         }
//     //         ResponseType::HandCard(card_id) => {
//     //             match res.player {
//     //                 PlayerType::Client => {
//     //                     // swap card
//     //                     // self.player_client
//     //                     //     .swap_card_on_hand(dragging_card_id, card_id);
//     //                 }
//     //                 PlayerType::Remote => {}
//     //             }
//     //         }
//     //         ResponseType::Hand => {}
//     //         _ => {
//     //             //drop
//     //             if let Some(pos) = self.cached_pos {
//     //                 let node = unsafe { card.node.assume_safe() };
//     //                 node.set_position(pos, false);
//     //             }
//     //         }
//     //     }
//     // }
// }
