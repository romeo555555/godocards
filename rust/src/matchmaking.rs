use crate::*;
use gdnative::api::Node;
use gdnative::prelude::*;
use std::collections::HashMap;

pub struct Match {
    history: Vec<Message>,
    players: HashMap<PlayerId, Player>, // TODO: Change collection //impl player_on_match_iter
    network: Network,
    hovereding: Hoverding,
    dragging: Dragging,
    sorting: Option<LineType>,
}

impl Match {
    pub fn new(owner: &Node, ctx: &mut Resources, player_data_handler: PlayerDataHandler) -> Self {
        switch_visible(owner, 1i64);
        let (mut network, mut match_info) = Network::new(player_data_handler);

        let MatchInfo {
            client_id,
            players,
            start_cards,
            opp_start_cards,
            bd_cards,
        } = match_info.receive();
        ctx.bd_cards.extend(bd_cards);
        network.client_id = client_id;
        let players =
            Resources::match_new_view(owner, ctx, client_id, players, opp_start_cards, start_cards);

        Self {
            history: Vec::with_capacity(100),
            players,
            network,
            hovereding: Hoverding::new(),
            dragging: Dragging::new(),
            sorting: None,
        }
    }

    pub fn input(&mut self, owner: &Node, ctx: &mut Resources) {
        let input = Input::godot_singleton();
        let sense = Sense::new(
            owner
                .cast::<CanvasItem>()
                .map(|node| node.get_global_mouse_position())
                .unwrap(),
            ctx.card_size(),
            Input::is_action_just_pressed(input, "mouse_button", false),
            Input::is_action_just_released(input, "mouse_button", false),
        );

        // let res = if gs.is_up_side(sense.mouse_y) {
        //     &self.player_remote
        // } else {
        //     &self.player_client
        // }
        // .input_handler(sense);

        if let Some(res) = self
            .players
            .iter()
            .find(|id_and_player| id_and_player.1.contains(sense))
            .map(|(id, player)| Response {
                item: player.contains_child(sense),
                player_id: *id,
                click_up: sense.click_up,
                click_down: sense.click_down,
            })
        {
            godot_print!("{:?}", res);
            if res.click_down {
                self.match_response(owner, ctx, res);
            } else if self.dragging.is_dragging() {
                if res.click_up {
                    self.match_drop(owner, ctx, res);
                }
                // else {
                //     res.match_dragging(owner, ctx, res);
                // }
            } else {
                //hovered
                if let ResponseType::TabelCard(card_id) | ResponseType::HandCard(card_id) = res.item
                {
                    self.hovereding.hovered(card_id);
                } else {
                }
            };
        }
        self.gui.run(ctx, sense);
    }
    pub fn match_response(&mut self, owner: &Node, ctx: &mut Resources, res: Response) {
        match res.item {
            ResponseType::TabelCard(card_id) => {
                // match res.player {
                //     PlayerType::Client=>{}
                //     PlayerType::Remote =>{}
                // }
            }
            ResponseType::HandCard(card_id) => {
                gui.drag(card_id);
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
                self.network.send_msg(Event::TakeCard(CardId::default()))
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
    pub fn match_dragging(&mut self, owner: &Node, ctx: &mut Resources, res: Response) {
        match res.item {
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

    pub fn match_drop(&mut self, owner: &Node, ctx: &mut Resources, res: Response) {
        match res.item {
            ResponseType::TabelCard(_) | ResponseType::Tabel => {
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
                let card_id = gui.get_dragging_id();
                self.network.send_msg(Event::CastCardOnTabel(card_id));
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
                gui.drop_without_target();
            }
        }
    }

    pub fn event(&mut self, owner: &Node, ctx: &mut Resources) {
        // self.state_update();
        // self.side_client.query(ctx);
        // self.side_remote.query(ctx);
        // self.player_client.update_position(res);
        // self.player_remote.update_position(res);
        self.players
            .values_mut()
            .for_each(|player| player.update_position(ctx));
        if let Some(msg) = self.network.receive_event() {
            let Message { player_id, event } = msg.clone();
            godot_print!("recive event : {:?}", event);
            match event {
                Event::TakeCard(card_id) => {
                    // self.player_client
                    //     .add_card_on_hand(res.create_card(owner));
                    // let card_id = res.create_card(card_name.clone());
                    // let side_player = self.get_side_player(commmand.player);
                    // match commmand.line {
                    //     LineType::Hand => side_player.hand.add_card(card_id),
                    //     LineType::Tabel => side_player.tabel.add_card(card_id),
                    //     _ => {}
                    // }

                    self.get_player(&player_id)
                        .add_card_on_hand(ctx.card_new(owner, card_id));
                }
                Event::CastCardOnTabel(card_id) => {
                    // if dragged { drop without pos} else{ drop without target?}
                    self.gui.drop();
                    self.get_player(&player_id).cast_on_tabel(card_id);
                }
                Event::BackCardOnHand(card_id) => {
                    // self.side_client.back_on_hand(card_id)
                }
                Event::ManaUpdate(count, color) => {
                    self.get_player(&player_id).mana_update(count, color);
                }
                Event::FlipCard(card_id, hash_card) => {
                    ctx.flip_card(owner, card_id, hash_card);
                }
                _ => {}
            }
            self.history.push(msg);
        }
    }
    pub fn get_player(&mut self, id: &PlayerId) -> &mut Player {
        self.players.get_mut(id).expect("dont have this player")
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
    pub fn get_dragging_id(&mut self) -> CardId {
        //??
        // self.cached_pos = None;
        // self.drop_back = false;
        self.select_card.unwrap()
    }
    pub fn run(&mut self, res: &mut Resources, pos: Vec2, card_offset: Vec2) {
        if let Some(select_id) = self.select_card {
            if self.drop_back {
                if let Some(cached_pos) = self.cached_pos {
                    let node = unsafe { res.get_card(select_id).node.assume_safe() };
                    // node.set_global_position(cached_pos + card_offset, false);
                    node.set_global_position(cached_pos, false);
                    self.select_card = None;
                    self.cached_pos = None;
                    self.drop_back = false;
                }
            } else {
                let node = unsafe { res.get_card(select_id).node.assume_safe() };
                if self.cached_pos.is_none() {
                    self.cached_pos = Some(node.global_position());
                    // node.set_scale(vec2(1.5, 1.5));
                    // // z-index -1
                }
                node.set_global_position(pos, false);
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

pub enum LineType {
    None,
    Tabel(PlayerId),
    Hand(PlayerId),
}
