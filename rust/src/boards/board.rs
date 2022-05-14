// use crate::misc::*;
use crate::game::GameSetting;
use crate::*;
use gdnative::api::Node;
use gdnative::prelude::*;
// use crate::{LineType::*,PlayerType::*, ResponseType::*,};
use std::collections::HashMap;
use std::ops::RangeBounds;
use std::slice::SliceIndex;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub struct Board {
    event: Vec<Message>,   //Vec<Command>,
    history: Vec<Message>, //Vec<Command>,
    players: HashMap<PlayerId, Player>,
    // player_remote: Player,
    // player_client: Player,
    client_id: PlayerId,
    // network_rx: Receiver<Message>,
    // message_tx: Sender<Message>,
    // end_botton_pos_and_size: (Vec2, Vec2),
    // game_state: State,
}

impl Board {
    pub fn new(
        owner: &Node,
        // player_remote: PlayerDataHandler,
        // player_client: PlayerDataHandler,
        // // players: HashMap<PlayerId, PlayerDataHandler>,
        // network_rx: Receiver<Message>,
        // message_tx: Sender<Message>,
        match_info: MatchInfo,
        resources: &mut Resources,
        gs: &GameSetting,
    ) -> Self {
        let MatchInfo {
            client_id,
            players,
            start_cards,
            opp_start_cards,
            bd_cards,
        } = match_info;
        resources.bd_cards.extend(bd_cards);
        let mut players = create::common_match(
            owner,
            gs.screen_rect,
            resources,
            client_id,
            players,
            opp_start_cards,
            start_cards,
            resources.card_size,
        );

        // let mut idx = 5;
        // players.values_mut().for_each(|player| {
        //     for i in 1..4 {
        //         player.add_card_on_hand(create::card(owner, resources, i + idx))
        //     }
        //     idx *= 2;
        // });
        Self {
            event: Vec::with_capacity(11),
            history: Vec::with_capacity(100),
            client_id,
            players,
            // network_rx,
            // message_tx,
        }
    }
    pub fn input_handler(
        &mut self,
        rendering: &mut Rendering,
        sense: Sense,
        gs: &GameSetting,
        owner: &Node,
        resources: &mut Resources,
    ) {
        // let res = if gs.is_up_side(sense.mouse_y) {
        //     &self.player_remote
        // } else {
        //     &self.player_client
        // }
        // .input_handler(sense);

        if let Some(player) = self.players.values().find(|player| player.contains(sense)) {
            let res = player.input_handler(sense);
            // godot_print!("{:?}", res);

            if res.click_up || res.click_down {
                if rendering.is_dragging() {
                    self.match_drop(res, rendering, rendering.get_dragging_id());
                } else {
                    self.match_response(res, rendering, owner, resources);
                }
            } else if rendering.is_dragging() {
                self.match_dragging(res, rendering);
            } else {
                //hovered
                if let ResponseType::TabelCard(card_id) | ResponseType::HandCard(card_id) = res.item
                {
                    rendering.hovered(card_id);
                } else {
                }
            };
        }
        // if is_key_down(KeyCode::Q) {
        //     let card_name = self.side_client.player.get_card_id();
        //     self.queue_command.push(
        //         CommandBuilder::default()
        //             .line(LineType::Hand)
        //             .build(PlayerType::Client, Event::add_card(card_name)),
        //     );
        // }
    }
    fn match_drop(&mut self, res: Response, rendering: &mut Rendering, dragging_card_id: CardId) {
        match res.item {
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
                rendering.drop_without_target();
            }
        }
    }
    fn match_dragging(&self, res: Response, rendering: &mut Rendering) {
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
    fn match_response(
        &mut self,
        res: Response,
        rendering: &mut Rendering,
        owner: &Node,
        resources: &mut Resources,
    ) {
        match res.item {
            ResponseType::TabelCard(card_id) => {
                // match res.player {
                //     PlayerType::Client=>{}
                //     PlayerType::Remote =>{}
                // }
            }
            ResponseType::HandCard(card_id) => {
                rendering.drag(card_id);
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

                godot_print!("{:?}", res);
                self.send_msg(Event::TakeCard(PlayerId::default()))
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
    // fn get_side_player(&mut self, player: PlayerType) -> &mut Side {
    //     match player {
    //         PlayerType::Client => &mut self.side_client,
    //         PlayerType::Remote => &mut self.side_remote,
    //     }
    // }
    fn get_player(&mut self, id: &PlayerId) -> &mut Player {
        self.players.get_mut(id).expect("dont have this player")
    }
    pub fn state_update(&mut self) {
        // self.game_state.next();
    }
    fn send_msg(&mut self, event: Event) {
        self.event
            .push(Message::Message(Msg::build(self.client_id, event)));
    }
    pub fn network_event(
        &mut self,
        owner: &Node,
        network: &mut Network,
        resources: &mut Resources,
    ) {
        // self.state_update();
        // self.side_client.query(ctx);
        // self.side_remote.query(ctx);
        // self.player_client.update_position(res);
        // self.player_remote.update_position(res);
        self.players
            .values_mut()
            .for_each(|player| player.update_position(resources));

        if let Some(msg) = self.event.pop() {
            godot_print!("Message Event");
            network.call(msg);
        }

        if let Some(Message::Message(msg)) = network.event_queue.try_receive() {
            godot_print!("TRY Receiver");
            let Msg { player_id, event } = msg.clone();
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
                        .add_card_on_hand(create::card(owner, resources, card_id));
                }
                Event::CastCardOnTabel(ref card_id) => {
                    // self.side_client.cast_on_tabel(card_id)
                } //match player
                Event::BackCardOnHand(card_id) => {
                    // self.side_client.back_on_hand(card_id)
                }
                Event::ManaUpdate(mana) => {
                    self.get_player(&player_id).mana_update(mana);
                }
                _ => {}
            }
            self.history.push(Message::Message(msg));
        }
    }
}
