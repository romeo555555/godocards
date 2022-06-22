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
use lazy_static::lazy_static;
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
mod layout;
use layout::*;

lazy_static! {
    static ref LAYOUT: Layout = Layout::new();
}
pub struct Gui {
    prefabs: Prefabs,
    players_view: HashMap<PlayerId, PlayerView>,
    cards_view: HashMap<CardId, CardView>,
}
impl Gui {
    pub fn new(
        owner: &Node,
        match_info: MatchInfo,
        server_api: ServerApi,
        // client_id: PlayerId,
    ) -> (Self, Store) {
        let mut prefabs = Prefabs::init(); //TODO:Up on ierarhi and in Option<Prefabs>
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

        let MatchInfo {
            client_id,
            players_state,
            players_data,
            cards,
            bd_cards,
        } = match_info;

        //TODO:new_1x1 return LAYOUT.2x2
        let players_view: HashMap<PlayerId, PlayerView> = players_data
            .into_iter()
            .map(|(id, player_data)| {
                if id == client_id {
                    (
                        id,
                        PlayerView::new(
                            match_scene.get_child(1),
                            player_data,
                            &LAYOUT.match1x1.client,
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
                            &LAYOUT.match1x1.opp,
                            PlayerType::Opp1,
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
                    CardView::FrontCardView(FrontCardView::new(owner, &mut prefabs, state.clone()))
                } else {
                    CardView::BackCardView(BackCardView::new(owner, &mut prefabs))
                };
                (*id, view)
            })
            .collect();
        let mut gui = Self {
            prefabs,
            players_view,
            cards_view,
        };
        players_state.iter().for_each(|(id, state)| {
            gui.sort_tabel(id, state.get_hand());
            gui.sort_hand(id, state.get_hand());
        });
        (
            gui,
            Store::new(players_state, cards, server_api, client_id, bd_cards),
        )
    }
    pub fn input(
        &mut self,
        sense: Sense,
        players_state: &HashMap<PlayerId, PlayerState>,
        selected_card: &mut SelectingCard,
    ) -> Option<Action> {
        // let drag_id = self.selecting_card.get_dragging_id();
        // if let Some(component_type) = self
        //     .players_view
        //     .iter()
        //     .find(|id_and_player| contains_rect(&sense.mouse_pos, &id_and_player.1.rect))
        //     .map(|(id, view)| {
        //         // let is_client = player.is_client();
        //         // Response {
        //         //     item: player.contains_child(
        //         //         sense,
        //         //         card_size,
        //         //         if is_client { drag_id } else { None },
        //         //     ),
        //         //     player_id: *id,
        //         //     click_up: sense.click_up,
        //         //     click_down: sense.click_down,
        //         //     is_client,
        //         // }

        //         //TODO: match player_type
        //         let layout = match view.player_type() {
        //             PlayerType::Client => &LAYOUT.match1x1.client,
        //             _ => &LAYOUT.match1x1.opp, // PlayerType::Friendly => {}
        //                                        // PlayerType::Opp => {}
        //         };
        //         view.contains_component(sense, players_state.get(id).unwrap(), layout)
        //     })
        {
            godot_print!("{:?}", component_type);
            return self.input_handler(component_type, sense, selected_card);
        }
        None
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
                //drop
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
    pub fn sort_hand(&mut self, player_id: &PlayerId, line: &Line) {
        let line_center = match self.players_view.get_mut(player_id).unwrap().player_type() {
            PlayerType::Client => LAYOUT.match1x1.client.hand.get_center(),
            _ => LAYOUT.match1x1.opp.hand.get_center(), // PlayerType::Friendly => {}
                                                        // PlayerType::Opp => {}
        };

        let card_size = LAYOUT.card.card_size;
        let card_indent = LAYOUT.card.card_indent;

        if let Some((mut x, y)) =
            alignment_line_point(line_center, line.len_float(), card_size, card_indent)
        {
            let x_indent = card_size.x + card_indent.x;

            for i in 0..line.len() {
                let card_id = line.get(i).unwrap();

                self.get_mut_card(&card_id).set_position(vec2(x, y));
                x += x_indent;
            }
        }
    }
    pub fn sort_tabel(&mut self, player_id: &PlayerId, line: &Line) {
        let line_center = match self.players_view.get_mut(player_id).unwrap().player_type() {
            PlayerType::Client => LAYOUT.match1x1.client.tabel.get_center(),
            _ => LAYOUT.match1x1.opp.tabel.get_center(), // PlayerType::Friendly => {}
                                                         // PlayerType::Opp => {}
        };

        let card_size = LAYOUT.card.card_size;
        let card_indent = LAYOUT.card.card_indent;

        if let Some((mut x, y)) =
            alignment_line_point(line_center, line.len_float(), card_size, card_indent)
        {
            let x_indent = card_size.x + card_indent.x;

            for i in 0..line.len() {
                let card_id = line.get(i).unwrap();

                self.get_mut_card(&card_id).set_position(vec2(x, y));
                x += x_indent;
            }
        }
    }

    pub fn create_card(&mut self, card_id: CardId, owner: &Node) {
        self.cards_view.insert(
            card_id,
            CardView::BackCardView(BackCardView::new(owner, &mut self.prefabs)),
        );
    }
    pub fn flip_card(&mut self, card_id: &CardId, owner: &Node, card_state: CardState) {
        let card_view = self
            .cards_view
            .get_mut(card_id)
            .expect("ERROR: Not foud card");
        if card_view.is_back() {
            let new_card_view = card_view.card_type_change(owner, &mut self.prefabs, card_state);
            self.cards_view.insert(*card_id, new_card_view);
        }
    }
    pub fn get_mut_card(&mut self, card_id: &CardId) -> &mut CardView {
        self.cards_view
            .get_mut(card_id)
            .expect("ERROR: Not foud card")
    }
    pub fn get_player_type(&self, player_id: &PlayerId) -> PlayerType {
        self.players_view.get(player_id).unwrap().player_type()
    }
    pub fn card_size(&self) -> Vec2 {
        LAYOUT.card.card_size
    }
}

pub struct PlayerView {
    player_type: PlayerType,
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
        layout: &LayoutPlayer,
        player_type: PlayerType,
        is_client: bool,
    ) -> Self {
        PlayerView {
            player_type,
            tabel: LineView::new(),
            hand: LineView::new(),
            deck: DeckView::new(player, layout.deck, player_data.deck_name),
            factories: FactoriesView::new(player, layout.factories, player_data.factories_name),
            equipment: EquipmentView::new(player, layout.equipment, player_data.equipment_name),
            character: CharacterView::new(player, layout.character, player_data.character_name),
            // avatar:
            // data: player_data.data,
            // is_client,
            // player_id: player_data.id.clone(),
        }
    }
    pub fn contains_component(
        &self,
        sense: Sense,
        player_state: &PlayerState,
        layout: &LayoutPlayer,
    ) -> ComponentType {
        // if match self.player_type {
        //     PlayerType::Client => sense.mouse_x > self.rect.center_x,
        //     // PlayerType::Remote => sense.mouse_x < self.rect.center_x,
        //     PlayerType::Remote => sense.mouse_x > self.rect.center_x,
        // }
        if sense.mouse_pos.x > self.rect.center_x {
            if contains_rect(&sense.mouse_pos, &layout.equipment) {
                return ComponentType::Equipment;
            } else if contains_rect(&sense.mouse_pos, &layout.character) {
                return ComponentType::Character;
            }
        } else if contains_rect(&sense.mouse_pos, &layout.deck) {
            return ComponentType::Deck;
        } else if contains_rect(&sense.mouse_pos, &layout.factories) {
            return ComponentType::Factories;
        }
        if contains_rect(&sense.mouse_pos, &layout.hand) {
            if let Some(card_id) = contains_cards_on_line(
                sense,
                player_state.get_hand(),
                layout.hand.get_center(),
                LAYOUT.card.card_size,
                LAYOUT.card.card_indent,
            ) {
                ComponentType::HandCard(card_id)
            } else {
                ComponentType::Hand
            };
        } else if contains_rect(&sense.mouse_pos, &layout.tabel) {
            return if let Some(card_id) = contains_cards_on_line(
                sense,
                player_state.get_tabel(),
                layout.tabel.get_center(),
                LAYOUT.card.card_size,
                LAYOUT.card.card_indent,
            ) {
                ComponentType::TabelCard(card_id)
            } else {
                ComponentType::Tabel
            };
        }
        ComponentType::None
    }
    pub fn player_type(&self) -> PlayerType {
        self.player_type
    }
}
pub struct LineView {
    // rect: Rect,
}
impl LineView {
    pub fn new() -> Self {
        Self {}
    }
}
pub struct EquipmentView {
    label_name: Ref<Label>,
    label_count: Ref<Label>,
}
impl EquipmentView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let items = player_component(player, "Items".to_owned(), rect, texture);
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
            label_name,
            label_count,
        }
    }
    pub fn update(&mut self) {}
}
pub struct CharacterView {
    label_name: Ref<Label>,
    label_healty: Ref<Label>,
}
impl CharacterView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let char = player_component(player, "Character".to_owned(), rect, texture);
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
            label_name,
            label_healty,
        }
    }
    pub fn update(&mut self) {}
}
pub struct DeckView {
    label_card_deck: Ref<Label>,
    label_dead_deck: Ref<Label>,
}

impl DeckView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let deck = player_component(player, "Deck".to_owned(), rect, texture);
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
            label_card_deck,
            label_dead_deck,
        }
    }
    pub fn update(&mut self) {}
}
pub struct FactoriesView {
    label_red: Ref<Label>,
    label_blue: Ref<Label>,
    label_green: Ref<Label>,
    label_white: Ref<Label>,
    label_black: Ref<Label>,
}
impl FactoriesView {
    pub fn new(player: Option<Ref<Node>>, rect: Rect, texture: String) -> Self {
        let builds = player_component(player, "Builds".to_owned(), rect, texture);
        Self {
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
    rect: Rect,
    texture: String,
) -> TRef<'a, TextureRect> {
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
    scene.set_global_position(rect.point(), false);
    scene.set_size(rect.size(), false);
    scene
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
