use common::{
    card::CardId,
    player::{Line, PlayerState},
};
use lazy_static::lazy_static;

use crate::{
    game::MatchType,
    gui::Gui,
    input::{
        client_click, client_dragging, client_drop, client_hover, opp_drop, ComponentType, Input,
        InputType, PlayerType,
    },
    store::Action,
    utils::{
        alignment_line_point, contains_card, contains_cards_on_line, contains_rect, vec2, Rect,
        Vec2,
    },
};
pub struct CardLayout {
    pub card_size: Vec2,
    pub card_indent: Vec2,
}
impl CardLayout {
    pub fn new() -> Self {
        let card_size = vec2(150., 180.);
        let card_indent = vec2(10., 0.);
        Self {
            card_size,
            card_indent,
        }
    }
}
lazy_static! {
    static ref CARD_LAYOUT: CardLayout = CardLayout::new();
}
pub struct Layout {
    // pub match1x1: LayoutMatch1x1,
    // match2x2: LayoutMatch2x2,
    // pub card: LayoutCard,
    match_type: MatchType,
    pub client: LayoutPlayer,
    pub opp1: LayoutPlayer,
    pub friend: Option<LayoutPlayer>,
    pub opp2: Option<LayoutPlayer>,
}
impl Layout {
    pub fn new(match_type: MatchType) -> Self {
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
        let screen_rect = Rect::new(0., 0., screen_size.x, screen_size.y);
        //if match_type == match1x1
        let side_up = screen_rect.up_split_side();
        let side_down = screen_rect.down_split_side();

        Self {
            match_type,
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
            opp1: LayoutPlayer {
                //indent = 20
                side: side_up,
                tabel: side_up.down_split_side(),
                hand: side_up.up_split_side(),
                deck: Rect::new(20., 20., 150., 180.),
                factories: Rect::new(180., 20., 150., 180.),
                equipment: Rect::new(900., 20., 150., 145.),
                character: Rect::new(1060., 20., 200., 200.),
            },
            friend: None,
            opp2: None,
            // match2x2: LayoutMatch2x2{
            //     client:LayoutPlayer::new()
            //     friend:LayoutPlayer::new()
            //     opp1:LayoutPlayer::new()
            //     opp2:LayoutPlayer::new()
            // },
            // card: LayoutCard::new(card_size, card_indent),
        }
    }
}
impl Layout {
    pub fn contains_player(&self, input: &Input) -> PlayerType {
        if contains_rect(&input.mouse_pos(), &self.client.side) {
            PlayerType::Client
        } else {
            PlayerType::Opp1
        }
    }
    pub fn input(
        &self,
        input: &Input,
        player_type: PlayerType,
        player_state: &PlayerState,
        // players_state: &HashMap<PlayerId, PlayerState>,
    ) -> Option<Action> {
        let (player_layout, player_type) = if contains_rect(&input.mouse_pos(), &self.client.side) {
            (&self.client, PlayerType::Client)
        } else {
            (&self.opp1, PlayerType::Opp1)
        };
        // if match self.player_type {
        //     PlayerType::Client => input.mouse_x > self.rect.center_x,
        //     // PlayerType::Remote => input.mouse_x < self.rect.center_x,
        //     PlayerType::Remote => input.mouse_x > self.rect.center_x,
        // }

        if let Some(component_type) = player_layout.contains_component(input, player_state) {
            self.input_handler(input, player_type, component_type)
        } else {
            None
        }
    }
    fn input_handler(
        &self,
        input: &Input,
        player_type: PlayerType,
        component_type: ComponentType,
    ) -> Option<Action> {
        match player_type {
            PlayerType::Client => match input.get_type() {
                InputType::Click => client_click(component_type),
                InputType::Dragging => client_dragging(component_type),
                InputType::Drop => client_drop(component_type, input),
                InputType::Hover => client_hover(component_type),
            },
            // PlayerType::Friendly => {}
            PlayerType::Opp1 | PlayerType::Opp2 => match input.get_type() {
                InputType::Drop => opp_drop(component_type, input),
                _ => None,
            },

            _ => None,
        }
    }
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
impl LayoutPlayer {
    fn contains_component(
        &self,
        input: &Input,
        player_state: &PlayerState,
    ) -> Option<ComponentType> {
        // if match self.player_type {
        //     PlayerType::Client => input.mouse_x > self.rect.center_x,
        //     // PlayerType::Remote => input.mouse_x < self.rect.center_x,
        //     PlayerType::Remote => input.mouse_x > self.rect.center_x,
        // }
        if input.mouse_pos().x > self.side.center_x {
            if contains_rect(&input.mouse_pos(), &self.equipment) {
                return Some(ComponentType::Equipment);
            } else if contains_rect(&input.mouse_pos(), &self.character) {
                return Some(ComponentType::Character);
            }
        } else if contains_rect(&input.mouse_pos(), &self.deck) {
            return Some(ComponentType::Deck);
        } else if contains_rect(&input.mouse_pos(), &self.factories) {
            return Some(ComponentType::Factories);
        }
        if contains_rect(&input.mouse_pos(), &self.hand) {
            return Some(
                if let Some(card_id) = self.contains_hand(input, player_state) {
                    ComponentType::HandCard(card_id)
                } else {
                    ComponentType::Hand
                },
            );
        } else if contains_rect(&input.mouse_pos(), &self.tabel) {
            return Some(
                if let Some(card_id) = self.contains_tabel(input, player_state) {
                    ComponentType::TabelCard(card_id)
                } else {
                    ComponentType::Tabel
                },
            );
        }
        None
    }
    pub fn contains_hand(&self, input: &Input, player_state: &PlayerState) -> Option<CardId> {
        contains_cards_on_line(
            input,
            player_state.get_hand(),
            self.hand.get_center(),
            CARD_LAYOUT.card_size,
            CARD_LAYOUT.card_indent,
        )
    }
    pub fn contains_tabel(&self, input: &Input, player_state: &PlayerState) -> Option<CardId> {
        contains_cards_on_line(
            input,
            player_state.get_tabel(),
            self.tabel.get_center(),
            CARD_LAYOUT.card_size,
            CARD_LAYOUT.card_indent,
        )
    }
    pub fn sort_hand(&self, player_state: &PlayerState, gui: &mut Gui) {
        let card_size = CARD_LAYOUT.card_size;
        let card_indent = CARD_LAYOUT.card_indent;
        let line = player_state.get_hand();

        if let Some((mut x, y)) = alignment_line_point(
            self.hand.get_center(),
            line.len_float(),
            card_size,
            card_indent,
        ) {
            let x_indent = card_size.x + card_indent.x;

            for i in 0..line.len() {
                let card_id = line.get(i).unwrap();

                gui.get_mut_card(&card_id).set_position(vec2(x, y));
                x += x_indent;
            }
        }
    }
    pub fn sort_tabel(&self, player_state: &PlayerState, gui: &mut Gui) {
        let card_size = CARD_LAYOUT.card_size;
        let card_indent = CARD_LAYOUT.card_indent;
        let line = player_state.get_tabel();

        if let Some((mut x, y)) = alignment_line_point(
            self.tabel.get_center(),
            line.len_float(),
            card_size,
            card_indent,
        ) {
            let x_indent = card_size.x + card_indent.x;

            for i in 0..line.len() {
                let card_id = line.get(i).unwrap();

                gui.get_mut_card(&card_id).set_position(vec2(x, y));
                x += x_indent;
            }
        }
    }
}
