use crate::game::GameSetting;
use crate::*;
use gdnative::{api::TextureRect, prelude::*};
use std::collections::HashMap;
pub type RefLabel = Ref<Label>;

pub struct Resources {
    // pub prefabs: Ref<Node>,
    // pub textures: HashMap<String, Ref<Texture>>,
    // players: HashMap<String, PlayerResources>,
    pub card_size: Vec2,
    card_prefab: Option<Ref<PackedScene>>,
    pub(crate) cards: HashMap<CardId, Card>, //cards on game
    pub(crate) bd_cards: HashMap<HashCard, CardStats>, //all game card stats
    players: HashMap<u64, Ref<Control>>,
    // player1: Option<Ref<Control>>,
    // player2: Option<Ref<Control>>,
}
impl Resources {
    pub const START_CARD_COUNT: usize = 30;
    // const ASSETS_COUNT: usize = 50;
    //TODO: Card_size
    //TODO: assets: Assets
    pub fn load(&mut self) {
        // let vec = vec!["Card", "Deck", "Build", "Item", "Avatar"];
        // for name in vec {
        //     self.prefabs.insert(
        //         name.to_owned(),
        //         load_scene(&["res://", name, ".tscn"].concat(), |scene| {
        //             Some(scene.claim())
        //         }), // .expect(&["Dont load prefab", name].join(" >>> ")),
        //     );
        // }
        // self.textures = assets.get_all();
        // self.card_size = unsafe{load("tampmini".to_owned()).assume_unique()}.size();
        self.card_prefab = load_scene("res://Card.tscn", |scene| Some(scene.claim()));
        self.card_size = vec2(150., 180.);
    }
    pub fn set_card_pos(&mut self, card_id: CardId, pos: Vec2) {
        unsafe { self.get_card(card_id).node.assume_safe() }.set_global_position(pos, false);
    }
    // pub fn set_card_pos(&mut self, id: u64, x: f32, y: f32) {
    //     self.cards.get_mut(&id).unwrap().set_pos(x, y);
    // }
    // pub fn get_card_cost(&self, card_id: u64) -> &Vec<ManaForm> {
    //     &self.cards.get(&card_id).unwrap().stats.cost
    // }
    pub fn get_card(&mut self, card_id: CardId) -> &mut Card {
        self.cards.get_mut(&card_id).expect("dfewf")
    }

    pub fn flip_card(&mut self, card_id: CardId, hash_card: HashCard) {
        let stats = self.bd_cards.get(&hash_card).unwrap().clone();
        let card = self.get_card(card_id);
        card.node.free();
        card.node.init
    }
}
impl Default for Resources {
    fn default() -> Self {
        Self {
            // textures: HashMap::with_capacity(Self::ASSETS_COUNT),
            card_size: Vec2::ZERO,
            card_prefab: None,
            cards: HashMap::with_capacity(Self::START_CARD_COUNT),
            bd_cards: HashMap::with_capacity(5),
            players: HashMap::with_capacity(4),
            // player1: None,
            // player2: None,
        }
    }
}

pub fn load_scene<F, T>(name: &str, mut f: F) -> Option<T>
where
    F: FnMut(TRef<PackedScene>) -> Option<T>,
{
    let scene = ResourceLoader::godot_singleton().load(name, "PackedScene", false)?;
    let scene = unsafe { scene.assume_safe() };
    let packed_scene = scene.cast::<PackedScene>()?;

    f(packed_scene)
}

// pub fn create_node_from_scene<T>(name: &str) -> Option<Ref<T>>
// where
//     T: GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
// {
//     load_scene(name, |scene| {
//         scene
//             .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
//             .map(|node| unsafe { node.assume_unique() })
//             .and_then(|node| node.cast::<T>())
//             .map(|node| node.into_shared())
//     })
// }

pub fn get_typed_node<O, F>(name: &str, owner: &Node, mut f: F)
where
    F: FnMut(TRef<O>),
    O: GodotObject + SubClass<Node>,
{
    let node = match owner
        .get_node(name)
        .map(|node| unsafe { node.assume_safe() })
        .and_then(|node| node.cast::<O>())
    {
        Some(it) => it,
        _ => {
            godot_print!("Couldn't find node {:?}", name);
            return;
        }
    };
    f(node)
}
fn load(name: String) -> Ref<Texture> {
    ResourceLoader::godot_singleton()
        .load(
            format!("res://assets/sprites/{}.png", name),
            "Texture",
            false,
        )
        .and_then(|res| res.cast::<Texture>())
        .expect("Couldn't load sprite texture")
}

pub mod create {
    use super::components::*;
    use crate::*;
    use std::collections::HashMap;
    // use card::Card;
    use gdnative::{api::TextureRect, prelude::*};

    pub fn card(owner: &Node, resources: &mut Resources, id: CardId) -> CardId {
        if let Some(prefab) = resources.card_prefab.take() {
            let card_obj = unsafe { prefab.assume_safe() };
            let card = card_obj
                .instance(0)
                .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
                .expect("Could not load player scene");
            // card.set_position(vec2(150., 180.) * vec2(hash_id as f32, hash_id as f32));
            // let pos = unsafe { card.get_child(0).unwrap().assume_safe() }
            //     .cast::<TextureRect>()
            //     .unwrap()
            //     .size();
            owner.add_child(card, false);
            //name load json
            //load stats
            resources.card_prefab.replace(card_obj.claim());
            resources.cards.insert(
                id,
                Card {
                    id,
                    node: card.claim(),
                    stats: None,
                },
            );
            id
        } else {
            panic!("Not found card_prefab")
        }
    }

    pub fn common_match(
        owner: &Node,
        rect: Rect,
        resources: &mut Resources,
        // player_remote: PlayerDataHandler,
        // player_client: PlayerDataHandler,
        client_id: PlayerId,
        players: HashMap<PlayerId, PlayerDataHandler>,
        opp_start_cards: HashMap<PlayerId, Vec<CardId>>,
        start_cards: Vec<(CardId, HashCard)>,
        card_size: Vec2,
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

        let mut players: HashMap<PlayerId, Player> = players
            .into_iter()
            .map(|(id, player_data)| {
                if id == client_id {
                    (
                        id,
                        player(
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
                        player(
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
            player.add_card_on_hand(create::card(owner, resources, card_id));
            resources.flip_card(card_id, hash_card);
        });
        opp_start_cards
            .into_iter()
            .for_each(|(player_id, vec_card_id)| {
                let player = players.get_mut(&player_id).unwrap();
                vec_card_id.into_iter().for_each(|card_id| {
                    player.add_card_on_hand(create::card(owner, resources, card_id));
                });
            });
        players

        // HashMap::from([
        //     (
        //         player_remote.id,
        //         player(
        //             match_scene.get_child(0),
        //             rect.up_split_side(),
        //             player_remote,
        //             card_size,
        //             rect.down_split_side(),
        //             rect.up_split_side(),
        //         ),
        //     ),
        //     (
        //         player_client.id,
        //         player(
        //             match_scene.get_child(1),
        //             rect.down_split_side(),
        //             player_client,
        //             card_size,
        //             rect.up_split_side(),
        //             rect.down_split_side(),
        //         ),
        //     ),
        // ])
    }
    // pub fn match_2x2(){}
    // pub fn match_two_faces(){}
    // pub fn player_two_faces(){}
    fn player(
        player: Option<Ref<Node>>,
        rect: Rect,
        player_data: PlayerDataHandler,
        card_size: Vec2,
        tabel_rect: Rect,
        hand_rect: Rect,
    ) -> Player {
        Player {
            player_id: player_data.id.clone(),
            rect,
            healty: 100,
            tabel: Tabel::new(tabel_rect, Player::CAPACITY_CARD_ON_HAND, card_size),
            hand: Hand::new(hand_rect, Player::CAPACITY_CARD_ON_HAND, card_size),
            deck: deck(player, player_data.deck_name),
            items: items(player, player_data.items_name),
            builds: builds(player, player_data.builds_name),
            character: chacracter(player, player_data.character_name),
            // avatar:
            data: player_data.data,
        }
    }

    fn deck(player: Option<Ref<Node>>, texture: String) -> Deck {
        let (deck, rect) = player_component(player, "Deck".to_owned(), texture);
        let label_card_count = deck
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        let label_dead_count = deck
            .get_child(0)
            .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
            .expect("Couldn't load sprite texture")
            .claim();
        godot_print!("Deck create: {}", rect);
        Deck::new(rect, label_card_count, label_dead_count)
    }
    fn builds(player: Option<Ref<Node>>, texture: String) -> Builds {
        let (builds, rect) = player_component(player, "Builds".to_owned(), texture);
        //builds
        // 0 - red
        // 1 - blue
        // 2 - green
        // 3 - white
        // 4 - black
        let nodes = vec!["Red", "Blue", "Green", "White", "Black"];
        let labels = nodes
            .iter()
            .map(|name| {
                builds
                    .get_node(name)
                    .and_then(|scene| unsafe { scene.assume_safe().cast::<Label>() })
                    .expect("Couldn't load sprite texture")
                    .claim()
            })
            .collect();
        godot_print!("Builds create: {}", rect);
        Builds::new(rect, labels)
    }
    fn items(player: Option<Ref<Node>>, texture: String) -> Items {
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
        Items::new(rect, label_name, label_count)
    }
    fn chacracter(player: Option<Ref<Node>>, texture: String) -> Character {
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
        Character::new(rect, label_name, label_healty)
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
}

// pub struct Assets {
//     card_textures: Vec<String>,
//     deck_textures: Vec<String>,
//     character: Vec<String>,
//     card_stats: String,
//     card_description: String,
//     card_stat_fiel: String,
// }
// impl Assets {
//     pub fn new(
//         card_textures: Vec<String>,
//         deck_textures: Vec<String>,
//         character: Vec<String>,
//         card_stats: String,
//         card_description: String,
//         card_stat_fiel: String,
//     ) -> Self {
//         Self {
//             card_textures,
//             deck_textures,
//             character,
//             card_stats,
//             card_description,
//             card_stat_fiel,
//         }
//     }
//     fn get_all(self) -> HashMap<String, Ref<Texture>> {
//         // let map = HashMap<String, Texture>::
//         let mut map = HashMap::with_capacity(50);
//         for name in self.card_textures {
//             map.insert(name.clone(), Assets::load(name));
//         }
//         for name in self.deck_textures {
//             map.insert(name.clone(), Assets::load(name));
//         }
//         for name in self.character {
//             map.insert(name.clone(), Assets::load(name));
//         }
//         let name = self.card_stats;
//         map.insert(name.clone(), Assets::load(name));
//         let name = self.card_description;
//         map.insert(name.clone(), Assets::load(name));
//         let name = self.card_stat_fiel;
//         map.insert(name.clone(), Assets::load(name));

//         map
//     }
//     fn load(name: String) -> Ref<Texture> {
//         ResourceLoader::godot_singleton()
//             .load(
//                 format!("res://assets/sprites/{}.png", name),
//                 "Texture",
//                 false,
//             )
//             .and_then(|res| res.cast::<Texture>())
//             .expect("Couldn't load sprite texture")
//     }
// }

// Assets::new(
// vec![
//     "card_back".to_owned(),
//     "tampmini".to_owned(),
//     "deckmini".to_owned(),
//     "deckmini1".to_owned(),
//     "deckmini2".to_owned(),
//     "builds".to_owned(),
//     "items".to_owned(),
//     "deck".to_owned(),
//     "avatar".to_owned(),
//     // "build",
//     // "spell",
// ],
// vec![
//     "card_deck_mini".to_owned(),
//     "dead_deck_mini".to_owned(),
//     "items_deck_mini".to_owned(),
//     "build_deck_mini".to_owned(),
// ],
// vec!["avatarmini".to_owned(), "avatarmini1".to_owned()],
// "stats".to_owned(),
// "stats_back".to_owned(),
// "stats_field".to_owned(),
// ));

