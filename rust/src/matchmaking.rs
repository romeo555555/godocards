use crate::*;
use gdnative::api::Node;
use gdnative::prelude::*;
use std::collections::HashMap;

pub struct Match {
    history: Vec<Message>,
    network: Network,
    gui: Gui,
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
        let gui = Gui::new(owner, ctx, client_id, players, opp_start_cards, start_cards);

        Self {
            history: Vec::with_capacity(100),
            network,
            gui,
        }
    }
    pub fn input(&mut self, owner: &Node, ctx: &mut Resources) {
        let card_size = ctx.card_size();
        let input = Input::godot_singleton();
        let sense = Sense::new(
            owner
                .cast::<CanvasItem>()
                .map(|node| node.get_global_mouse_position())
                .unwrap(),
            card_size,
            Input::is_action_just_pressed(input, "mouse_button", false),
            Input::is_action_just_released(input, "mouse_button", false),
        );

        // let res = if gs.is_up_side(sense.mouse_y) {
        //     &self.player_remote
        // } else {
        //     &self.player_client
        // }
        // .input_handler(sense);

        if let Some(res) = self.gui.check_input(sense, card_size) {
            godot_print!("{:?}", res);
            res.handler(owner, ctx, &mut self.gui, &mut self.network)
        }

        self.gui.run(ctx, sense);
    }
    pub fn event(&mut self, owner: &Node, ctx: &mut Resources) {
        // self.state_update();
        // self.side_client.query(ctx);
        // self.side_remote.query(ctx);
        // self.player_client.update_position(res);
        // self.player_remote.update_position(res);
        ////////////////////////////////// self.players
        //     .values_mut()
        //     .for_each(|player| player.update_position(ctx));
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

                    self.gui
                        .get_player(&player_id)
                        .add_card_on_hand(ctx.card_new(owner, card_id));
                }
                Event::CastCardOnTabel(card_id) => {
                    // if dragged { drop without pos} else{ drop without target?}
                    self.gui.drop(ctx);
                    self.gui.get_player(&player_id).cast_on_tabel(card_id);
                }
                Event::BackCardOnHand(card_id) => {
                    // self.side_client.back_on_hand(card_id)
                }
                Event::ManaUpdate(count, color) => {
                    self.gui.get_player(&player_id).mana_update(count, color);
                }
                Event::FlipCard(card_id, hash_card) => {
                    ctx.flip_card(owner, card_id, hash_card);
                }
                _ => {}
            }
            self.history.push(msg);
        }
    }
}
