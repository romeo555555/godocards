use gdnative::api::Node;

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
    pub fn match_drop(self, ctx: &mut Resources, selected: &mut SelectCard) {
        match self.item {
            //ctx.get_dragging_id()
            // ResponseType::TabelCard(_) | ResponseType::Tabel => {
            //     //cast to tabel
            //     // let card_cost = rendering.get_card_cost(fit_card_id);
            //     //                     if self.side_client.player.try_pay_mana(card_cost) {
            //     //                         self.queue_command.push(
            //     //                             CommandBuilder::default().line(LineType::Hand).build(
            //     //                                 PlayerType::Client,
            //     //                                 Event::cast_on_tabel(fit_card_id),
            //     //                             ),
            //     //                         );
            //     //                         rendering.drop();
            //     //                     }

            //     // LineType::Tabel => {
            //     //                 let card_cost = rendering.get_card_cost(fit_card_id);
            //     //                 if self.side_client.player.try_pay_mana(card_cost) {
            //     //                     self.queue_command.push(
            //     //                         CommandBuilder::default().line(LineType::Hand).build(
            //     //                             PlayerType::Client,
            //     //                             Event::cast_on_tabel(fit_card_id),
            //     //                         ),
            //     //                     );
            //     //                     rendering.drop();
            //     //                 }
            //     //             }
            // }
            // ResponseType::HandCard(card_id) => {
            //     match res.player {
            //         PlayerType::Client => {
            //             // swap card
            //             // self.player_client
            //             //     .swap_card_on_hand(dragging_card_id, card_id);
            //         }
            //         PlayerType::Remote => {}
            //     }
            // }
            // ResponseType::Hand => {}
            _ => {
                //drop
                selected.drop_without_target();
            }
        }
    }
    pub fn match_dragging(self, ctx: &mut Resources, selected: &mut SelectCard) {
        match self.item {
            ResponseType::TabelCard(card_id) => {
                //put card
            }
            ResponseType::Tabel => {
                //cast to tabel
            }
            ResponseType::HandCard(card_id) => {
                //put card
            }
            ResponseType::Hand => { //
                 // if Client swap card
            }
            _ => {}
        }
    }
    pub fn match_response(self, owner: &Node, ctx: &mut Resources, selected: &mut SelectCard) {
        match self.item {
            ResponseType::TabelCard(card_id) => {
                // match res.player {
                //     PlayerType::Client=>{}
                //     PlayerType::Remote =>{}
                // }
            }
            ResponseType::HandCard(card_id) => {
                selected.drag(card_id);
                // match res.player {
                //     PlayerType::Client => {
                //         //drag
                //         rendering.drag(card_id);
                //     }
                //     PlayerType::Remote => {}
                // }
            }
            ResponseType::Deck => {
                //+card and show card deck count / card dead deck

                // godot_print!("{:?}", res);
                selected.send_msg(Event::TakeCard(PlayerId::default()))
                // let side_player = self.get_side_player(res.player);
                // if let DeckType::BuildDeck = deck_type {
                //     side_player.player.add_mana(Mana::Red(2));
                // } else {
                //     let card_name = side_player.player.get_card_id();
                //     self.queue_command.push(
                //         CommandBuilder::default()
                //             .line(LineType::Hand)
                //             .build(res.player, Event::add_card(card_name)),
                //     );
                // self.query_command.push(match res.player {
                //     PlayerType::Client => Command::AddCardClientHand("deckmini1".to_string()), //self.side_client.add_card_on_hand(),
                //     PlayerType::Remote => Command::AddCardRemoteHand("deckmini1".to_string()),
                // });
            }
            ResponseType::Builds => {
                //show builds
                // self.players
                //     .get_mut(&self.client_id)
                //     .expect("player_client not found")
                //     .add_card_on_hand(create::card(owner, resources, 10000));
            }
            ResponseType::Items => {
                //show items
            }
            ResponseType::Character => {
                //show character descripton
            }
            //click on board
            ResponseType::Tabel => {}
            ResponseType::Hand => {}
            ResponseType::None => {}
        }
    }
}
