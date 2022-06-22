use std::collections::HashMap;

use common::player::{PlayerId, PlayerState};

use crate::{
    input_action::{ComponentType, Sense},
    utils::{contains_cards_on_line, contains_rect, vec2, Rect, Vec2},
};

pub struct Layout {
    pub match1x1: LayoutMatch1x1,
    // match2x2: LayoutMatch2x2,
    pub card: LayoutCard,
}
impl Layout {
    pub fn new() -> Self {
        // let screen_size = owner
        //             .cast::<CanvasItem>()
        //             .map(|node| node.get_viewport_rect())
        //             .map(|viewport| {
        //                 godot_print!(
        //                     "_{}-{}_ is screen pos! //// _{}-{}_ is screen size!",
        //                     viewport.position.x,
        //                     viewport.position.y,
        //                     viewport.size.x,
        //                     viewport.size.y,
        //                 );
        //                 viewport.size
        //             })
        //             .unwrap();
        let screen_size = vec2(1280., 720.);
        let card_size = vec2(150., 180.);
        let card_indent = vec2(10., 0.);
        let screen_rect = Rect::new(0., 0., screen_size.x, screen_size.y);
        let side_up = screen_rect.up_split_side();
        let side_down = screen_rect.down_split_side();

        Self {
            match1x1: LayoutMatch1x1 {
                opp: LayoutPlayer {
                    //indent = 20
                    side: side_up,
                    tabel: side_up.down_split_side(),
                    hand: side_up.up_split_side(),
                    deck: Rect::new(20., 20., 150., 180.),
                    factories: Rect::new(180., 20., 150., 180.),
                    equipment: Rect::new(900., 20., 150., 145.),
                    character: Rect::new(1060., 20., 200., 200.),
                },
                client: LayoutPlayer {
                    //indent = 20
                    side: side_down,
                    tabel: side_down.up_split_side(),
                    hand: side_down.down_split_side(),
                    deck: Rect::new(20., 520., 150., 180.),
                    factories: Rect::new(180., 520., 150., 180.),
                    equipment: Rect::new(900., 520., 150., 145.),
                    character: Rect::new(1060., 520., 200., 200.),
                },
            },
            // match2x2: LayoutMatch2x2{
            //     client:LayoutPlayer::new()
            //     friend:LayoutPlayer::new()
            //     opp1:LayoutPlayer::new()
            //     opp2:LayoutPlayer::new()
            // },
            card: LayoutCard::new(card_size, card_indent),
        }
    }
}
pub struct LayoutMatch1x1 {
    pub client: LayoutPlayer,
    pub opp: LayoutPlayer,
}
impl Layout {
    pub fn contains_component_on_match1x1(
        &self,
        sense: Sense,
        players_state: &HashMap<PlayerId, PlayerState>,
    ) -> ComponentType {
        let player_layout = if contains_rect(&sense.mouse_pos, &self.match1x1.client.side) {
            &self.match1x1.client
        } else {
            &self.match1x1.opp
        };
        // if match self.player_type {
        //     PlayerType::Client => sense.mouse_x > self.rect.center_x,
        //     // PlayerType::Remote => sense.mouse_x < self.rect.center_x,
        //     PlayerType::Remote => sense.mouse_x > self.rect.center_x,
        // }
        if sense.mouse_pos.x > player_layout.side.center_x {
            if contains_rect(&sense.mouse_pos, &player_layout.equipment) {
                return ComponentType::Equipment;
            } else if contains_rect(&sense.mouse_pos, &player_layout.character) {
                return ComponentType::Character;
            }
        } else if contains_rect(&sense.mouse_pos, &player_layout.deck) {
            return ComponentType::Deck;
        } else if contains_rect(&sense.mouse_pos, &player_layout.factories) {
            return ComponentType::Factories;
        }
        if contains_rect(&sense.mouse_pos, &player_layout.hand) {
            if let Some(card_id) = contains_cards_on_line(
                sense,
                players_state.get_hand(),
                player_layout.hand.get_center(),
                self.card.card_size,
                self.card.card_indent,
            ) {
                ComponentType::HandCard(card_id)
            } else {
                ComponentType::Hand
            };
        } else if contains_rect(&sense.mouse_pos, &player_layout.tabel) {
            return if let Some(card_id) = contains_cards_on_line(
                sense,
                player_state.get_tabel(),
                player_layout.tabel.get_center(),
                self.card.card_size,
                self.card.card_indent,
            ) {
                ComponentType::TabelCard(card_id)
            } else {
                ComponentType::Tabel
            };
        }
        ComponentType::None
    }
}
pub struct LayoutMatch2x2 {
    pub client: LayoutPlayer,
    pub friend: LayoutPlayer,
    pub opp1: LayoutPlayer,
    pub opp2: LayoutPlayer,
}
pub struct LayoutPlayer {
    pub side: Rect,
    pub tabel: Rect,
    pub hand: Rect,
    pub deck: Rect,
    pub factories: Rect,
    pub equipment: Rect,
    pub character: Rect,
}
pub struct LayoutCard {
    pub card_size: Vec2,
    pub card_indent: Vec2,
}
impl LayoutCard {
    pub fn new(card_size: Vec2, card_indent: Vec2) -> Self {
        Self {
            card_size,
            card_indent,
        }
    }
}
