use crate::*;
use common::{card::CardId, player::PlayerId};
use gdnative::prelude::{Input as GodoInput, *};

#[derive(Clone, Copy)]
pub enum SelectedState {
    None,
    Hoverd(CardId),
    Dragging(CardId),
}
pub struct Input {
    mouse_pos: Vec2,
    click_down: bool,
    click_up: bool,
    select_card: SelectedState,
    cached_hovered_card: Option<(CardId, Vec2)>,
}
impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            click_down: false,
            click_up: false,
            select_card: SelectedState::None,
            cached_hovered_card: None,
        }
    }
    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }
    pub fn click_down(&self) -> bool {
        self.click_down
    }
    pub fn click_up(&self) -> bool {
        self.click_up
    }
    pub fn get_type(&self) -> InputType {
        if self.click_down {
            InputType::Click
        } else if let SelectedState::Dragging(_) = self.select_card {
            if self.click_up {
                InputType::Drop
            } else {
                InputType::Dragging
            }
        } else {
            InputType::Hover
        }
    }
    pub fn update(&mut self, owner: &Node) {
        // off if windiw game not focused
        // let (mouse_x, mouse_y) = (0., 0.); //mouse_position();
        let input = GodoInput::godot_singleton();
        self.mouse_pos = owner
            .cast::<CanvasItem>()
            .map(|node| node.get_global_mouse_position())
            .unwrap();
        self.click_down = GodoInput::is_action_just_pressed(input, "mouse_button", false);
        self.click_up = GodoInput::is_action_just_released(input, "mouse_button", false);
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
    pub fn get_id_if_dragging(&self) -> Option<CardId> {
        if let SelectedState::Dragging(card_id) = self.select_card {
            return Some(card_id);
        }
        None
    }
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
    pub fn update_selected(&mut self, gui: &mut Gui) {
        let card_offset = vec2(0., 30.);
        let card_size = vec2(150., 180.);
        match self.get_state() {
            SelectedState::Dragging(card_id) => {
                if let Some((cached_card, pos)) = self.cached_hovered() {
                    // reset
                    if !contains_card(self.mouse_pos, card_size, pos.x, pos.y) {
                        gui.get_mut_card(cached_card).hovered_off(*pos);
                        self.cached_hovered_clean();
                    }
                }
                //dragging
                gui.get_mut_card(&card_id).set_position(self.mouse_pos);
            }
            SelectedState::Hoverd(ref card_id) => {
                if let Some((cached_card, pos)) = self.cached_hovered() {
                    //reset + set
                    if card_id != cached_card {
                        gui.get_mut_card(cached_card).hovered_off(*pos);
                        let cached_pos = gui.get_mut_card(card_id).hovered_on(card_offset);
                        self.set_cached_hovered(*card_id, cached_pos);
                    }
                } else {
                    //set
                    let cached_pos = gui.get_mut_card(card_id).hovered_on(card_offset);
                    self.set_cached_hovered(*card_id, cached_pos);
                }
                self.drop()
            }
            SelectedState::None => {
                if let Some((cached_card, pos)) = self.cached_hovered() {
                    // reset
                    if !contains_card(self.mouse_pos, card_size, pos.x, pos.y) {
                        gui.get_mut_card(cached_card).hovered_off(*pos);
                        self.cached_hovered_clean();
                    }
                }
            }
        }
    }
}
// #[derive(Clone, Copy)]
// pub struct Sense {
//     // pub card_size: Vec2,
//     // pub mouse_x: f32,
//     // pub mouse_y: f32,
//     pub mouse_pos: Vec2,
//     pub click_down: bool,
//     pub click_up: bool,
//     pub is_dragging: bool,
// }
// impl Sense {
//     pub fn new(mouse_pos: Vec2, is_dragging: bool) -> Self {
//         // off if windiw game not focused
//         // let (mouse_x, mouse_y) = (0., 0.); //mouse_position();
//         let input = GodoInput::godot_singleton();
//         Self {
//             mouse_pos,
//             click_down: GodoInput::is_action_just_pressed(input, "mouse_button", false),
//             click_up: GodoInput::is_action_just_released(input, "mouse_button", false),
//             is_dragging,
//         }
//     }
//     pub fn input_type(&self) -> InputType {
//         if self.click_down {
//             InputType::Click
//         } else if self.is_dragging {
//             if self.click_up {
//                 InputType::Drop
//             } else {
//                 InputType::Dragging
//             }
//         } else {
//             InputType::Hover
//         }
//     }
// }
//     pub fn contains_rect(&self, rect: &Rect) -> bool {
//         self.mouse_x >= rect.left()
//             && self.mouse_x < rect.right()
//             && self.mouse_y < rect.bottom()
//             && self.mouse_y >= rect.top()
//     }
//     pub fn contains_card(&self, x: f32, y: f32) -> bool {
//         self.mouse_x >= x
//             && self.mouse_x < x + self.card_size.x
//             && self.mouse_y < y + self.card_size.y
//             && self.mouse_y >= y
//     }
//     pub fn mouse_position(&self) -> Vec2 {
//         vec2(self.mouse_x, self.mouse_y)
//     }
// }

#[derive(Clone, Copy, Debug)]
pub enum SelectAction {
    Drag(CardId),
    Hover(CardId),
    // Dragging(CardId),
    Drop,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComponentType {
    // None,
    TabelCard(CardId),
    HandCard(CardId),
    Tabel,
    Hand,
    // HandWitchExclude(CardId),
    Deck,
    Factories,
    Equipment,
    Character,
    //Avatar
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerType {
    Client,
    Friendly,
    Opp1,
    Opp2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputType {
    Click,
    Dragging,
    Drop,
    Hover,
}

pub fn client_click(component_type: ComponentType) -> Option<Action> {
    match component_type {
        ComponentType::TabelCard(card_id) => {
            // match res.player {
            //     PlayerType::Client=>{}
            //     PlayerType::Remote =>{}
            // }
        }
        ComponentType::HandCard(card_id) => {
            // client_flag.insert(FlagsForUpdate::HAND);
            // selected_card.drag(card_id);
            return Some(Action::Select(SelectAction::Drag(card_id)));
            // match res.player {
            //     PlayerType::Client => {
            //         //drag
            //         rendering.drag(card_id);
            //     }
            //     PlayerType::Remote => {}
            // }
        }
        ComponentType::Deck => {
            //+card and show card deck count / card dead deck

            // godot_print!("{:?}", res);
            return Some(Action::Client(ClientAction::TakeCard));
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
        ComponentType::Factories => {
            //show builds
            // self.players
            //     .get_mut(&self.client_id)
            //     .expect("player_client not found")
            //     .add_card_on_hand(create::card(owner, resources, 10000));
        }
        ComponentType::Equipment => {
            //show items
        }
        ComponentType::Character => {
            //show character descripton
        }
        // InputType::TabelCard(card_id) => {
        //             // match res.player {
        //             //     PlayerType::Client=>{}
        //             //     PlayerType::Remote =>{}
        //             // }
        //         }
        //         InputType::HandCard(card_id) => {
        //             gui.drag(ctx, card_id);
        //             // match res.player {
        //             //     PlayerType::Client => {
        //             //         //drag
        //             //         rendering.drag(card_id);
        //             //     }
        //             //     PlayerType::Remote => {}
        //             // }
        //         }
        //click on board
        ComponentType::Tabel => {}
        ComponentType::Hand => {}
        _ => {}
    }
    None
}
pub fn client_drop(component_type: ComponentType, input: &Input) -> Option<Action> {
    match component_type {
        ComponentType::Tabel => {
            //cast to tabel
            // let card_cost = rendering.get_card_cost(fit_card_id);
            //                     if self.side_client.player.try_pay_mana(card_cost) {
            //                         self.queue_command.push(
            //                             CommandBuilder::default().line(LineType::Hand).build(
            //                                 PlayerType::Client,
            //                                 Event::cast_on_tabel(fit_card_id),
            //                             ),
            //                         );
            //                         rendering.drop();
            if let Some(card_id) = input.get_id_if_dragging() {
                return Some(Action::Client(ClientAction::CastCardOnTabel(card_id)));
            }
        }

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
            // client_flag.insert(FlagsForUpdate::HAND);
            // selected_card.drop();
            return Some(Action::Select(SelectAction::Drop));
        }
    }
    None
}
pub fn client_dragging(component_type: ComponentType) -> Option<Action> {
    match component_type {
        ComponentType::Tabel => {
            // ::TabelCard(card_id) => {
            //put card
        }
        ComponentType::Hand => { //
             // ResponseType::HandCard(card_id) => {
             //put card
             // if Client swap card
        }
        _ => {}
    }
    None
}
pub fn client_hover(component_type: ComponentType) -> Option<Action> {
    if let ComponentType::TabelCard(card_id) | ComponentType::HandCard(card_id) = component_type {
        ////////////////////////gui.hovered(card_id);
        return Some(Action::Select(SelectAction::Hover(card_id)));
    }
    None
}
pub fn friend_click() {}
pub fn friend_drop() {}
pub fn friend_dragging() {}
pub fn friend_hover() {}

pub fn opp_click() {}
pub fn opp_drop(component_type: ComponentType, input: &Input) -> Option<Action> {
    match component_type {
        ComponentType::Tabel => {
            //cast to tabel
            // let card_cost = rendering.get_card_cost(fit_card_id);
            //                     if self.side_client.player.try_pay_mana(card_cost) {
            //                         self.queue_command.push(
            //                             CommandBuilder::default().line(LineType::Hand).build(
            //                                 PlayerType::Client,
            //                                 Event::cast_on_tabel(fit_card_id),
            //                             ),
            //                         );
            //                         rendering.drop();
            if let Some(card_id) = input.get_id_if_dragging() {
                return Some(Action::Client(ClientAction::CastCardOnTabel(card_id)));
            }
        }

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
            // client_flag.insert(FlagsForUpdate::HAND);
            // selected_card.drop();
            return Some(Action::Select(SelectAction::Drop));
        }
    }
    None
}
pub fn opp_dragging() {}
pub fn opp_hover() {}

// #[derive(Clone, Copy, Debug)]
// pub enum ResponseType {
//     None,
//     TabelCard(CardId),
//     HandCard(CardId),
//     Tabel,
//     Hand,
//     Deck,
//     Builds,
//     Items,
//     Character,
//     //Avatar
// }
// #[derive(Debug)]
// pub struct Response {
//     pub item: ResponseType,
//     pub click_down: bool,
//     pub click_up: bool,
//     pub player_id: PlayerId,
//     pub is_client: bool,
// }
// impl Response {
//     pub fn new(
//         item: ResponseType,
//         player_id: PlayerId,
//         click_up: bool,
//         click_down: bool,
//         is_client: bool,
//     ) -> Response {
//         //build_response
//         Response {
//             item,
//             click_down,
//             click_up,
//             player_id,
//             is_client,
//         }
//     }
//     pub fn handler(self, owner: &Node, ctx: &mut Resources, gui: &mut Gui, network: &mut Network) {
//         if self.is_client {
//             if self.click_down {
//                 self.match_response(owner, ctx, gui, network);
//             } else if gui.is_dragging() {
//                 if self.click_up {
//                     self.match_drop(owner, ctx, gui, network);
//                 } else {
//                     self.match_dragging(owner, ctx, gui, network);
//                 }
//             }
//         } else {
//             //hovered
//             if let ResponseType::TabelCard(card_id) | ResponseType::HandCard(card_id) = self.item {
//                 ////////////////////////gui.hovered(card_id);
//             } else {
//             }
//         };
//     }
//     pub fn match_response(
//         self,
//         owner: &Node,
//         ctx: &mut Resources,
//         gui: &mut Gui,
//         network: &mut Network,
//     ) {
//         match self.item {
//             ResponseType::TabelCard(card_id) => {
//                 // match res.player {
//                 //     PlayerType::Client=>{}
//                 //     PlayerType::Remote =>{}
//                 // }
//             }
//             ResponseType::HandCard(card_id) => {
//                 gui.drag(ctx, card_id);
//                 // match res.player {
//                 //     PlayerType::Client => {
//                 //         //drag
//                 //         rendering.drag(card_id);
//                 //     }
//                 //     PlayerType::Remote => {}
//                 // }
//             }
//             ResponseType::Deck => {
//                 //+card and show card deck count / card dead deck

//                 // godot_print!("{:?}", res);
//                 network.send_msg(Event::TakeCard(CardId::default()))
//                 // let side_player = self.get_side_player(res.player);
//                 // if let DeckType::BuildDeck = deck_type {
//                 //     side_player.player.add_mana(Mana::Red(2));
//                 // } else {
//                 //     let card_name = side_player.player.get_card_id();
//                 //     self.queue_command.push(
//                 //         CommandBuilder::default()
//                 //             .line(LineType::Hand)
//                 //             .build(res.player, Event::add_card(card_name)),
//                 //     );
//                 // self.query_command.push(match res.player {
//                 //     PlayerType::Client => Command::AddCardClientHand("deckmini1".to_string()), //self.side_client.add_card_on_hand(),
//                 //     PlayerType::Remote => Command::AddCardRemoteHand("deckmini1".to_string()),
//                 // });
//             }
//             ResponseType::Builds => {
//                 //show builds
//                 // self.players
//                 //     .get_mut(&self.client_id)
//                 //     .expect("player_client not found")
//                 //     .add_card_on_hand(create::card(owner, resources, 10000));
//             }
//             ResponseType::Items => {
//                 //show items
//             }
//             ResponseType::Character => {
//                 //show character descripton
//             }
//             //click on board
//             ResponseType::Tabel => {}
//             ResponseType::Hand => {}
//             ResponseType::None => {}
//         }
//     }
//     pub fn match_dragging(
//         self,
//         owner: &Node,
//         ctx: &mut Resources,
//         gui: &mut Gui,
//         network: &mut Network,
//     ) {
//         match self.item {
//             ResponseType::TabelCard(card_id) => {
//                 //put card
//             }
//             ResponseType::Tabel => {
//                 //cast to tabel
//             }
//             ResponseType::HandCard(card_id) => {
//                 //put card
//             }
//             ResponseType::Hand => { //
//                  // if Client swap card
//             }
//             _ => {}
//         }
//     }

//     pub fn match_drop(
//         self,
//         owner: &Node,
//         ctx: &mut Resources,
//         gui: &mut Gui,
//         network: &mut Network,
//     ) {
//         match self.item {
//             ResponseType::TabelCard(_) | ResponseType::Tabel => {
//                 //cast to tabel
//                 // let card_cost = rendering.get_card_cost(fit_card_id);
//                 //                     if self.side_client.player.try_pay_mana(card_cost) {
//                 //                         self.queue_command.push(
//                 //                             CommandBuilder::default().line(LineType::Hand).build(
//                 //                                 PlayerType::Client,
//                 //                                 Event::cast_on_tabel(fit_card_id),
//                 //                             ),
//                 //                         );
//                 //                         rendering.drop();
//                 if let Some(card_id) = gui.get_dragging_id() {
//                     network.send_msg(Event::CastCardOnTabel(card_id));
//                 }
//             }

//             //     // LineType::Tabel => {
//             //     //                 let card_cost = rendering.get_card_cost(fit_card_id);
//             //     //                 if self.side_client.player.try_pay_mana(card_cost) {
//             //     //                     self.queue_command.push(
//             //     //                         CommandBuilder::default().line(LineType::Hand).build(
//             //     //                             PlayerType::Client,
//             //     //                             Event::cast_on_tabel(fit_card_id),
//             //     //                         ),
//             //     //                     );
//             //     //                     rendering.drop();
//             //     //                 }
//             //     //             }
//             // }
//             // ResponseType::HandCard(card_id) => {
//             //     match res.player {
//             //         PlayerType::Client => {
//             //             // swap card
//             //             // self.player_client
//             //             //     .swap_card_on_hand(dragging_card_id, card_id);
//             //         }
//             //         PlayerType::Remote => {}
//             //     }
//             // }
//             // ResponseType::Hand => {}
//             _ => {
//                 //drop
//                 gui.drop(ctx);
//             }
//         }
//     }
// }
