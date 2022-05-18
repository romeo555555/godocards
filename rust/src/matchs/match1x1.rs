use crate::*;
use gdnative::api::Node;
use gdnative::prelude::*;
use std::collections::HashMap;

pub struct Match1x1 {
    history: Vec<Message>,
    players: HashMap<PlayerId, Player>, //impl player_on_match_iter
    network: Network,
    selected: SelectCard,
    // client_id: PlayerId,
    // event: Vec<Message>,
    // hovereding: Hoverding,
    // dragging: Dragging,
}

impl Match1x1 {
    pub fn new(owner: &Node, ctx: &mut Resources, player_data_handler: PlayerDataHandler) -> Self {
        switch_visible(owner, 1i64);
        let mut network = Network::new(player_data_handler);

        let match_info = loop {
            if let Message::MatchInfo(match_info) = network.event_queue.receive() {
                break match_info;
            }
        };

        let MatchInfo {
            client_id,
            players,
            start_cards,
            opp_start_cards,
            bd_cards,
        } = match_info;

        ctx.bd_cards.extend(bd_cards);
        let players = Self::new_view(owner, ctx, client_id, players, opp_start_cards, start_cards);

        Self {
            history: Vec::with_capacity(100),
            players,
            network,
            selected: SelectCard::new(client_id),
            // client_id,
            // event: Vec::with_capacity(11),
            // hovereding: Hoverding::default(),
            // dragging: Dragging::default(),
        }
    }
    fn new_view(
        owner: &Node,
        ctx: &mut Resources,
        client_id: PlayerId,
        players: HashMap<PlayerId, PlayerDataHandler>,
        opp_start_cards: HashMap<PlayerId, Vec<CardId>>,
        start_cards: Vec<(CardId, HashCard)>,
    ) -> HashMap<PlayerId, Player> {
        let match_scene = ResourceLoader::godot_singleton()
            .load("res://Match.tscn", "PackedScene", false)
            .and_then(|res| {
                let res = unsafe { res.assume_thread_local() };
                res.cast::<PackedScene>()
            })
            .and_then(|packed_scene| packed_scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED))
            .and_then(|scene| {
                let scene = unsafe { scene.assume_safe() };
                scene.cast::<Node2D>()
            })
            .expect("Could not load player scene");
        owner.add_child(match_scene, false);
        let rect = ctx.screen_rect();
        let card_size = ctx.card_size();

        let mut players: HashMap<PlayerId, Player> = players
            .into_iter()
            .map(|(id, player_data)| {
                if id == client_id {
                    (
                        id,
                        Player::new(
                            match_scene.get_child(1),
                            rect.down_split_side(),
                            player_data,
                            card_size,
                            rect.up_split_side(),
                            rect.down_split_side(),
                        ),
                    )
                } else {
                    (
                        id,
                        Player::new(
                            match_scene.get_child(0),
                            rect.up_split_side(),
                            player_data,
                            card_size,
                            rect.down_split_side(),
                            rect.up_split_side(),
                        ),
                    )
                }
            })
            .collect();

        start_cards.into_iter().for_each(|(card_id, hash_card)| {
            let player = players.get_mut(&client_id).unwrap();
            player.add_card_on_hand(ctx.card_new(owner, card_id));
            ctx.flip_card(owner, card_id, hash_card);
        });
        opp_start_cards
            .into_iter()
            .for_each(|(player_id, vec_card_id)| {
                let player = players.get_mut(&player_id).unwrap();
                vec_card_id.into_iter().for_each(|card_id| {
                    player.add_card_on_hand(ctx.card_new(owner, card_id));
                });
            });
        players
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

        if let Some(player) = self.players.values().find(|player| player.contains(sense)) {
            let res = player.input_handler(sense);
            // godot_print!("{:?}", res);

            if res.click_up || res.click_down {
                if self.selected.is_dragging() {
                    res.match_drop(ctx, &mut self.selected);
                } else {
                    res.match_response(owner, ctx, &mut self.selected);
                }
            } else if self.selected.is_dragging() {
                res.match_dragging(ctx, &mut self.selected);
            } else {
                //hovered
                if let ResponseType::TabelCard(card_id) | ResponseType::HandCard(card_id) = res.item
                {
                    self.selected.hovered(card_id);
                } else {
                }
            };
        }
        self.selected.run(ctx, sense);
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
        if let Some(msg) = self.selected.pop_event() {
            godot_print!("Message Event");
            self.network.call(msg);
        }
        if let Some(Message::Message(msg)) = self.network.event_queue.try_receive() {
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
                        .add_card_on_hand(ctx.card_new(owner, card_id));
                }
                Event::CastCardOnTabel(ref card_id) => {
                    // self.side_client.cast_on_tabel(card_id)
                } //match player
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
            self.history.push(Message::Message(msg))
        }
    }
    pub fn get_player(&mut self, id: &PlayerId) -> &mut Player {
        self.players.get_mut(id).expect("dont have this player")
    }
}
