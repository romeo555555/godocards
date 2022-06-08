pub mod card;
mod line;
pub mod player;
mod system;

use crate::*;
use card::*;
use gdnative::api::{Label, Node, Node2D, PackedScene, ResourceLoader, TextureRect};
use gdnative::object::{Ref, TRef};
use gdnative::prelude::Shared;
use gdnative::{api::Texture, prelude::godot_print};
use line::*;
use nanoserde::{DeBin, SerBin};
pub use player::*;
use std::collections::{HashMap, VecDeque};
use std::ops::{Index, IndexMut};
use std::{cmp::Ordering, ops::Add};
use system::*;

// pub enum RenderEvent {
//     Hovered(CardId),
//     Drag(CardId),
//     Drop(CardId),
//     SortPosLine,
// }

pub struct Gui {
    selecting_card: SelectingCard,
    players: HashMap<PlayerId, Player>, // TODO: Change collection //impl player_on_match_iter
    line_for_update: Option<(PlayerId, LineType)>,
    //render_qeu
    // render_event: Vec<RenderEvent>,
}
impl Gui {
    pub fn new(
        owner: &Node,
        ctx: &mut Resources,
        client_id: PlayerId,
        players: HashMap<PlayerId, PlayerDataHandler>,
        opp_start_cards: HashMap<PlayerId, Vec<CardId>>,
        start_cards: Vec<(CardId, HashCard)>,
        // ) -> HashMap<PlayerId, Player> {
    ) -> Self {
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
        let rect_up = rect.up_split_side();
        let rect_down = rect.down_split_side();

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
                            rect_down.up_split_side(),
                            rect_down.down_split_side(),
                            true,
                        ),
                    )
                } else {
                    (
                        id,
                        Player::new(
                            match_scene.get_child(0),
                            rect.up_split_side(),
                            player_data,
                            rect_up.down_split_side(),
                            rect_up.up_split_side(),
                            false,
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

        Self {
            selecting_card: SelectingCard::new(),
            players,
            line_for_update: None,
        }
    }
    pub fn check_input(&mut self, sense: Sense, card_size: Vec2) -> Option<Response> {
        let drag_id = self.selecting_card.get_dragging_id();
        self.players
            .iter()
            .find(|id_and_player| id_and_player.1.contains(sense))
            .map(|(id, player)| {
                let is_client = player.is_client();
                Response {
                    item: player.contains_child(
                        sense,
                        card_size,
                        if is_client { drag_id } else { None },
                    ),
                    player_id: *id,
                    click_up: sense.click_up,
                    click_down: sense.click_down,
                    is_client,
                }
            })
    }
    pub fn get_player(&mut self, id: &PlayerId) -> &mut Player {
        self.players.get_mut(id).expect("dont have this player")
    }
    // pub fn draw(&mut self, ctx: &mut Resources, sense: Sense) {
    //     if let Some(event) = self.render_event.pop() {
    //         match event {
    //             RenderEvent::Hovered(card_id) => {
    //                 self.hovereding.select_id = Some(card_id);
    //             }
    //             RenderEvent::Drag(card_id) => {
    //                 self.dragging.select_card = Some(card_id);
    //             }
    //             RenderEvent::Drop(card_id) => {}
    //             RenderEvent::SortPosLine => {}
    //         }
    //     }
    //     self.run(ctx, sense);
    // }
    pub fn run(&mut self, ctx: &mut Resources, sense: Sense) {
        self.selecting_card.run(ctx, sense);
    }
    pub fn sort_line(&mut self, ctx: &mut Resources) {
        if let Some((player_id, line_type)) = self.line_for_update.take() {
            let drag_id = self.selecting_card.get_dragging_id();
            self.get_player(&player_id)
                .sort_line(ctx, line_type, drag_id)
        }
    }
    pub fn drag(&mut self, ctx: &mut Resources, card_id: CardId) {
        self.selecting_card.drag(card_id);
        self.sort_line(ctx)
    }
    pub fn drop(&mut self, ctx: &mut Resources) {
        self.selecting_card.drop();
        self.sort_line(ctx)
    }
    pub fn hover(&mut self, card_id: CardId) {
        self.selecting_card.hovered(card_id);
    }
    pub fn is_dragging(&self) -> bool {
        self.selecting_card.is_dragging()
    }
    pub fn get_dragging_id(&mut self) -> Option<CardId> {
        self.selecting_card.get_dragging_id()
    }
    // pub fn drop(&mut self) {
    //     self.drag.drop();
    // }
    // pub fn drop_without_target(&mut self) {
    //     self.drag.drop_without_target();
    // }
}
