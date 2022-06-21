use common::{
    card::{CardId, CardState},
    game_match::MatchInfo,
    mana::{Mana, ManaColor, ManaForm},
    player::{Line, PlayerData, PlayerId, PlayerState},
};
mod card;
use card::*;
use gdnative::{
    api::{CanvasItem, Control, Label},
    core_types::Color,
    object::{Ref, TRef},
    prelude::*,
};
use std::{
    collections::HashMap,
    slice::{Iter, SliceIndex},
};

use crate::utils::{vec2, Rect};

use crate::*;
use gdnative::api::{Node, ResourceLoader, TextureRect};
use gdnative::prelude::Shared;
use gdnative::{api::Texture, prelude::godot_print};
use nanoserde::{DeBin, SerBin};
use std::collections::VecDeque;
use std::{cmp::Ordering, ops::Add};

pub struct Gui {
    players_view: HashMap<PlayerId, PlayerView>,
    cards_view: HashMap<CardId, CardView>,
}
impl Gui {
    pub fn new(
        owner: &Node,
        res: &mut Resources,
        match_info: MatchInfo,
        server_api: ServerApi,
        // client_id: PlayerId,
    ) -> (
        Self,
        Store, // HashMap<PlayerId, FlagsForUpdate>,
               // HashMap<PlayerId, PlayerState>,
               // HashMap<CardId, Option<CardState>>
    ) {
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
        let rect = res.screen_rect();
        let rect_up = rect.up_split_side();
        let rect_down = rect.down_split_side();

        let MatchInfo {
            client_id,
            players_state,
            players_data,
            cards,
            bd_cards,
        } = match_info;
        res.bd_cards.extend(bd_cards);

        let players_view: HashMap<PlayerId, PlayerView> = players_data
            .into_iter()
            .map(|(id, player_data)| {
                if id == client_id {
                    (
                        id,
                        PlayerView::new(
                            match_scene.get_child(1),
                            player_data,
                            rect_down,
                            rect_down.up_split_side(),
                            rect_down.down_split_side(),
                            PlayerType::Client,
                            true,
                        ),
                    )
                } else {
                    (
                        id,
                        PlayerView::new(
                            match_scene.get_child(0),
                            player_data,
                            rect_up,
                            rect_up.down_split_side(),
                            rect_up.up_split_side(),
                            PlayerType::Opp,
                            false,
                        ),
                    )
                }
            })
            .collect();

        let cards_view = cards
            .iter()
            .map(|(id, state)| {
                let view = if let Some(state) = state {
                    CardView::FrontCardView(FrontCardView::new(owner, res, state.clone()))
                } else {
                    CardView::BackCardView(BackCardView::new(owner, res))
                };
                (*id, view)
            })
            .collect();
        (
            Self {
                players_view,
                cards_view,
            },
            Store::new(
                players_state
                    .keys()
                    .map(|id| (*id, FlagsForUpdate::all()))
                    .collect(),
                players_state,
                cards,
                server_api,
                client_id,
            ),
        )
    }
    pub fn input(
        &mut self,
        sense: Sense,
        players_state: &HashMap<PlayerId, PlayerState>,
        res: &mut Resources,
        selected_card: &mut SelectingCard,
    ) -> Option<Action> {
        // let drag_id = self.selecting_card.get_dragging_id();
        self.players_view
            .iter()
            .find(|id_and_player| contains_rect(&sense.mouse_pos, &id_and_player.1.rect))
            .map(|(id, view)| {
                // let is_client = player.is_client();
                // Response {
                //     item: player.contains_child(
                //         sense,
                //         card_size,
                //         if is_client { drag_id } else { None },
                //     ),
                //     player_id: *id,
                //     click_up: sense.click_up,
                //     click_down: sense.click_down,
                //     is_client,
                // }

                let card_size = res.card_size();
                let card_indent = res.card_indent();
                //TODO: match player_type
                let players_state = players_state.get(id).unwrap();
                let input_type = view.contains_component(
                    sense,
                    players_state,
                    card_size,
                    card_indent,
                    selected_card.get_id_if_dragging(),
                );
                self.input_handler(input_type, sense, selected_card)
            })
            .flatten()
    }
    fn input_handler(
        &mut self,
        input_type: ComponentType,
        sense: Sense,
        selected_card: &mut SelectingCard,
    ) -> Option<Action> {
        if sense.click_down {
            //click
            match input_type {
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
                ComponentType::None => {}
                _ => {}
            }
        } else if selected_card.is_dragging() {
            if sense.click_up {
                match input_type {
                    ComponentType::TabelCard(_) | ComponentType::Tabel => {
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
                        if let Some(card_id) = selected_card.get_id_if_dragging() {
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
            } else {
                match input_type {
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
            }
        }
        if let ComponentType::TabelCard(card_id) | ComponentType::HandCard(card_id) = input_type {
            ////////////////////////gui.hovered(card_id);
            return Some(Action::Select(SelectAction::Hover(card_id)));
        }
        None
    }
    // pub fn update(
    //     &mut self,
    //     player_id: &PlayerId,
    //     player_state: &PlayerState,
    //     flags: &FlagsForUpdate,
    //     res: &mut Resources,
    //     exclude_card: Option<CardId>,
    // ) {
    //     // let player_view = self.players_view.get_mut(player_id).unwrap();

    //     // if player_state.flags.deck {
    //     //     player_view.deck.update();
    //     // }
    //     // if player_state.flags.factories {
    //     //     player_view.factories.update();
    //     // }
    //     // if player_state.flags.character {
    //     //     player_view.character.update();
    //     // }
    //     // if player_state.flags.equipment {
    //     //     player_view.equipment.update();
    //     // }

    //     if flags.contains(FlagsForUpdate::HAND) {
    //         let center = self
    //             .players_view
    //             .get_mut(player_id)
    //             .unwrap()
    //             .hand
    //             .rect
    //             .get_center();
    //         let line = player_state.get_hand();
    //         self.sort_line(
    //             if exclude_card.is_some() {
    //                 line.len_float() - 1.
    //             } else {
    //                 line.len_float()
    //             },
    //             line.get_cards(),
    //             center,
    //             res.card_size(),
    //             res.card_indent(),
    //             exclude_card,
    //         );
    //     }
    //     if flags.contains(FlagsForUpdate::TABEL) {
    //         let center = self
    //             .players_view
    //             .get_mut(player_id)
    //             .unwrap()
    //             .tabel
    //             .rect
    //             .get_center();
    //         let line = player_state.get_tabel();
    //         self.sort_line(
    //             line.len_float(),
    //             line.get_cards(),
    //             center,
    //             res.card_size(),
    //             res.card_indent(),
    //             exclude_card,
    //         );
    //     }
    // }

    pub fn sort_line(
        &mut self,
        line_len: f32,
        // mut line_iter: Iter<CardId>,
        line: &[CardId],
        line_center: Vec2,
        card_size: Vec2,
        card_indent: Vec2,
        exclude_card: Option<CardId>,
    ) {
        if let Some(exclude_card) = exclude_card {
            if let Some((mut x, y)) =
                alignment_line_point(line_center, line_len - 1., card_size, card_indent)
            {
                let x_indent = card_size.x + card_indent.x;

                for i in 0..line.len() {
                    let card_id = line.get(i).unwrap();
                    if *card_id == exclude_card {
                        continue;
                    }

                    self.get_mut_card(card_id).set_position(vec2(x, y));
                    x += x_indent;
                }
                // for (idx, card_id) in line_iter.enumerate() {
                //             if contains_card(sense.mouse_pos, card_size, x, y) {
                //                 return Some((idx, *card_id));
                //             }
                //             //         // godot_print!(
                //             //         //     "card input @:{} pos: {}-{},,, card_size x:{}, y;{}",
                //             //         //     card_id,
                //             //         //     x,
                //             //         //     y,
                //             //         //     sense.card_size.x,
                //             //         //     sense.card_size.y,
                //             //         // );
                //             x += x_indent;
                //         }
                // // let mut iter = iter();
                // for _ in 0..line_iter.len() {
                //     self.get_mut_card(
                //         line_iter
                //             .next()
                //             .map(|card_id| {
                //                 if *card_id == exclude_card {
                //                     return line_iter.next().unwrap();
                //                 }
                //                 card_id
                //             })
                //             .unwrap(),
                //     )
                //     .set_position(vec2(x, y));
                //     // godot_print!(
                //     //     "card set_pos @:{} pos: {}-{},,, card_size x:{}, y;{}",
                //     //     *card_id,
                //     //     x,
                //     //     y,
                //     //     card_size.x,
                //     //     card_size.y,
                //     // );
                //     x += x_indent;
                // }
            }
        } else if let Some((mut x, y)) =
            alignment_line_point(line_center, line_len, card_size, card_indent)
        {
            let x_indent = card_size.x + card_indent.x;
            // for card_id in line_iter {
            //     self.get_mut_card(card_id).set_position(vec2(x, y));
            //     // godot_print!(
            //     //     "card set_pos @:{} pos: {}-{},,, card_size x:{}, y;{}",
            //     //     card_id,
            //     //     x,
            //     //     y,
            //     //     card_size.x,
            //     //     card_size.y,
            //     // );
            //     x += x_indent;
            // }
            for i in 0..line.len() {
                let card_id = line.get(i).unwrap();

                self.get_mut_card(card_id).set_position(vec2(x, y));
                x += x_indent;
            }
        }
    }
    pub fn get_mut_card(&mut self, card_id: &CardId) -> &mut CardView {
        self.cards_view
            .get_mut(card_id)
            .expect("ERROR: Not foud card")
    }
}

pub struct PlayerView {
    rect: Rect,
    players_type: PlayerType,
    tabel: LineView,
    hand: LineView,
    deck: DeckView,
    factories: FactoriesView,
    equipment: EquipmentView,
    character: CharacterView,
    //avatar
}
impl PlayerView {
    pub const CAPACITY_CARD_ON_TABEL: usize = 8;
    pub const CAPACITY_CARD_ON_HAND: usize = 8;
    pub const CAPACITY_CARD_ON_DECK: usize = 8;
    pub const CAPACITY_CARD_ON_ITEMS: usize = 8;
    pub const CAPACITY_CARD_ON_BUILDS: usize = 8;
    pub fn new(
        player: Option<Ref<Node>>,
        player_data: PlayerData,
        rect: Rect,
        tabel_rect: Rect,
        hand_rect: Rect,
        players_type: PlayerType,
        is_client: bool,
    ) -> Self {
        PlayerView {
            // player_id: player_data.id.clone(),
            rect,
            players_type,
            tabel: LineView::new(tabel_rect),
            hand: LineView::new(hand_rect),
            deck: DeckView::new(player, player_data.deck_name),
            factories: FactoriesView::new(player, player_data.factories_name),
            equipment: EquipmentView::new(player, player_data.equipment_name),
            character: CharacterView::new(player, player_data.character_name),
            // avatar:
            // data: player_data.data,
            // is_client,
        }
    }
    pub fn contains_component(
        &self,
        sense: Sense,
        player_state: &PlayerState,
        card_size: Vec2,
        card_indent: Vec2,
        exclude_card: Option<CardId>,
    ) -> ComponentType {
        // if match self.player_type {
        //     PlayerType::Client => sense.mouse_x > self.rect.center_x,
        //     // PlayerType::Remote => sense.mouse_x < self.rect.center_x,
        //     PlayerType::Remote => sense.mouse_x > self.rect.center_x,
        // }
        if sense.mouse_pos.x > self.rect.center_x {
            if contains_rect(&sense.mouse_pos, &self.equipment.rect) {
                return ComponentType::Equipment;
            } else if contains_rect(&sense.mouse_pos, &self.character.rect) {
                return ComponentType::Character;
            }
        } else if contains_rect(&sense.mouse_pos, &self.deck.rect) {
            return ComponentType::Deck;
        } else if contains_rect(&sense.mouse_pos, &self.factories.rect) {
            return ComponentType::Factories;
        }
        if contains_rect(&sense.mouse_pos, &self.hand.rect) {
            let line = player_state.get_hand();
            return if let Some(exclude_card) = exclude_card {
                if let Some((idx, card_id)) = contains_cards_on_line(
                    sense,
                    line.len_float() - 1.,
                    line.iter(),
                    self.hand.rect.get_center(),
                    card_size,
                    card_indent,
                ) {
                    if exclude_card == card_id {
                        return if let Some(next_card_id) = line.get(idx + 1) {
                            ComponentType::HandCard(next_card_id)
                        } else {
                            ComponentType::Hand
                        };
                    }
                    ComponentType::HandCard(card_id)
                } else {
                    ComponentType::Hand
                }
            } else if let Some((idx, card_id)) = contains_cards_on_line(
                sense,
                line.len_float(),
                line.iter(),
                self.hand.rect.get_center(),
                card_size,
                card_indent,
            ) {
                ComponentType::HandCard(card_id)
            } else {
                ComponentType::Hand
            };
        } else if contains_rect(&sense.mouse_pos, &self.tabel.rect) {
            let line = player_state.get_tabel();
            return if let Some((_, card_id)) = contains_cards_on_line(
                sense,
                line.len_float(),
                line.iter(),
                self.tabel.rect.get_center(),
                card_size,
                card_indent,
            ) {
                ComponentType::TabelCard(card_id)
            } else {
                ComponentType::Tabel
            };
        }
        ComponentType::None
    }
}
pub struct LineView {
    rect: Rect,
}
impl LineView {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}
pub struct EquipmentView {
    rect: Rect,
    label_name: Ref<Label>,
    label_count: Ref<Label>,
}
impl EquipmentView {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
        let (items, rect) = player_component(player, "Items".to_owned(), texture);
        let label_name = items
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_count = items
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        godot_print!("Items create: {}", rect);
        Self {
            rect,
            label_name,
            label_count,
        }
    }
    pub fn update(&mut self) {}
}
pub struct CharacterView {
    rect: Rect,
    label_name: Ref<Label>,
    label_healty: Ref<Label>,
}
impl CharacterView {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
        let (char, rect) = player_component(player, "Character".to_owned(), texture);
        let label_name = char
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_healty = char
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        godot_print!("Character create: {}", rect);
        Self {
            rect,
            label_name,
            label_healty,
        }
    }
    pub fn update(&mut self) {}
}
pub struct DeckView {
    rect: Rect,
    label_card_deck: Ref<Label>,
    label_dead_deck: Ref<Label>,
}

impl DeckView {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
        let (deck, rect) = player_component(player, "Deck".to_owned(), texture);
        let label_card_deck = deck
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_dead_deck = deck
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        godot_print!("Deck create: {}", rect);
        Self {
            rect,
            label_card_deck,
            label_dead_deck,
        }
    }
    pub fn update(&mut self) {}
}
pub struct FactoriesView {
    rect: Rect,
    label_red: Ref<Label>,
    label_blue: Ref<Label>,
    label_green: Ref<Label>,
    label_white: Ref<Label>,
    label_black: Ref<Label>,
}
impl FactoriesView {
    pub fn new(player: Option<Ref<Node>>, texture: String) -> Self {
        let (builds, rect) = player_component(player, "Builds".to_owned(), texture);

        Self {
            rect,
            label_red: builds
                .get_node("Red")
                .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                .expect("Couldn't load sprite texture")
                .claim(),
            label_blue: builds
                .get_node("Red")
                .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                .expect("Couldn't load sprite texture")
                .claim(),
            label_green: builds
                .get_node("Red")
                .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                .expect("Couldn't load sprite texture")
                .claim(),
            label_white: builds
                .get_node("Red")
                .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                .expect("Couldn't load sprite texture")
                .claim(),
            label_black: builds
                .get_node("Red")
                .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                .expect("Couldn't load sprite texture")
                .claim(),
        }
        // godot_print!("Builds create: {}", rect);
    }
    pub fn update(&mut self) {}
    // pub fn update(&mut self, count: u64, color: ManaColor) {
    //     if let Some(label) = self.labels.get_mut(match color {
    //         ManaColor::Red => 0,
    //         ManaColor::Blue => 1,
    //         ManaColor::Green => 2,
    //         ManaColor::White => 3,
    //         ManaColor::Black => 4,
    //     }) {
    //         unsafe { label.assume_safe() }.set_text(count.to_string());
    //     }
    //     // .unwrap();
    // }
}
fn player_component<'a>(
    player: Option<Ref<Node, Shared>>,
    name: String,
    texture: String,
) -> (TRef<'a, TextureRect>, Rect) {
    let scene = player
        .and_then(|scene| unsafe { scene.assume_safe() }.get_node(name))
        .and_then(|scene| unsafe { scene.assume_safe().cast::<TextureRect>() })
        .map(|scene| {
            scene.set_texture(
                ResourceLoader::godot_singleton()
                    .load(
                        format!("res://assets/sprites/{}.png", texture),
                        "Texture",
                        false,
                    )
                    .and_then(|res| res.cast::<Texture>())
                    .expect("Couldn't load sprite texture"),
            );
            scene
        })
        .expect("Couldn't load sprite texture");
    let pos = scene.global_position();
    let size = scene.size();
    (scene, Rect::new(pos.x, pos.y, size.x, size.y))
}
// fn avatar( player: Option<Ref<Node>>, texture: String, pos: Vec2) -> Deck {
//     let deck = player_component(player, "Avatar", texture);
//     deck.set_position(pos, false);
//     let label_card_count = deck
//         .get_child(0)
//         .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
//         .expect("Couldn't load sprite texture")
//         .claim();
//     let label_dead_count = deck
//         .get_child(0)
//         .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
//         .expect("Couldn't load sprite texture")
//         .claim();
//     let size = deck.size();
//     Deck::new(
//         Rect::new(pos.x, pos.x, size.x, size.y),
//         label_card_count,
//         label_dead_count,
//     )
// }
