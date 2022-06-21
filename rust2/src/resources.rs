use crate::*;
use gdnative::{api::TextureRect, prelude::*};
use std::collections::HashMap;
pub type RefLabel = Ref<Label>;

pub struct Resources {
    // pub textures: HashMap<String, Ref<Texture>>,
    pub(crate) prefab_card: Option<Ref<PackedScene>>,
    pub(crate) prefab_card_unit: Option<Ref<PackedScene>>,
    pub(crate) prefab_card_spell: Option<Ref<PackedScene>>,
    pub(crate) prefab_mana: Option<Ref<PackedScene>>,
    pub(crate) bd_cards: HashMap<HashCard, CardStats>, // bd all game card stats
    pub(crate) config: Config,

    // cards: HashMap<CardId, Card>,               //cards on current game
    // cards_view: HashMap<CardId, CardStatsView>, //maybe add type damage? and add texture color fon
    players_node: HashMap<u64, Ref<Control>>,
}
impl Resources {
    pub const START_CARD_COUNT: usize = 30;
    // const ASSETS_COUNT: usize = 50;
    //TODO: Card_size
    //TODO: assets: Assets
    pub fn load_prefabs_and_config(&mut self, owner: &Node) {
        //config: Config
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
        self.prefab_card = load_scene("res://Card.tscn", |scene| Some(scene.claim()));
        self.prefab_card_unit = load_scene("res://Unit.tscn", |scene| Some(scene.claim()));
        self.prefab_card_spell = load_scene("res://Spell.tscn", |scene| Some(scene.claim()));
        self.prefab_mana = load_scene("res://Mana.tscn", |scene| Some(scene.claim()));
        // self.card_size = vec2(150., 180.);
        self.config = Config::new(owner, vec2(150., 180.), vec2(10., 0.));
    }
    // pub fn set_card_pos(&mut self, card_id: CardId, pos: Vec2) {
    //     unsafe { self.get_card(card_id).node.assume_safe() }.set_global_position(pos, false);
    // }
    // pub fn set_card_pos(&mut self, id: u64, x: f32, y: f32) {
    //     self.cards.get_mut(&id).unwrap().set_pos(x, y);
    // }
    // pub fn get_card_cost(&self, card_id: u64) -> &Vec<ManaForm> {
    //     &self.cards.get(&card_id).unwrap().stats.cost
    // }
    // pub fn get_card(&mut self, card_id: CardId) -> &mut Card {
    //     self.cards.get_mut(&card_id).expect("dfewf")
    // }

    // pub fn flip_card(&mut self, owner: &Node, card_id: CardId, hash_card: HashCard) {
    //     self.card_type_change(
    //         owner,
    //         card_id,
    //         self.bd_cards.get(&hash_card).unwrap().clone(),
    //     );
    // }
    pub fn screen_rect(&self) -> Rect {
        self.config.screen_rect
    }
    pub fn card_size(&self) -> Vec2 {
        self.config.card_size
    }
    pub fn card_indent(&self) -> Vec2 {
        self.config.card_indent
    }
    //     pub fn card_type_change(&mut self, owner: &Node, card_id: CardId, stats: CardStats) {
    //         let card_node = unsafe { self.get_card(card_id).node.assume_unique() };
    //         let pos = card_node.global_position();
    //         card_node.queue_free();

    //         let card_node = match stats.card_type {
    //             CardType::Unit(_) => {
    //                 let prefab_card_unit = self.prefab_card_unit.take().unwrap();
    //                 let prefab_obj = unsafe { prefab_card_unit.assume_safe() };
    //                 let card_unit_node = prefab_obj
    //                     .instance(0)
    //                     .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
    //                     .expect("Could not load player scene");
    //                 card_unit_node.set_global_position(pos, false);
    //                 owner.add_child(card_unit_node, false);
    //                 //name load json
    //                 //load stats

    //                 self.prefab_card_unit.replace(prefab_obj.claim());
    //                 card_unit_node
    //             }
    //             _ => {
    //                 let prefab_card_spell = self.prefab_card_spell.take().unwrap();
    //                 let prefab_obj = unsafe { prefab_card_spell.assume_safe() };
    //                 let card_spell_node = prefab_obj
    //                     .instance(0)
    //                     .and_then(|scene| unsafe { scene.assume_safe() }.cast::<Control>())
    //                     .expect("Could not load player scene");
    //                 card_spell_node.set_global_position(pos, false);
    //                 // let pos = unsafe { card.get_child(0).unwrap().assume_safe() }
    //                 //     .cast::<TextureRect>()
    //                 //     .unwrap()
    //                 //     .size();
    //                 owner.add_child(card_spell_node, false);
    //                 //name load json
    //                 //load stats
    //                 self.prefab_card_spell.replace(prefab_obj.claim());
    //                 card_spell_node
    //             }
    //         };
    //         let (card_stats_view, card) = self.card_new_view(card_node, stats);
    //         self.cards_view.insert(card_id, card_stats_view);
    //         self.cards.insert(card_id, card);
    //     }
}
impl Default for Resources {
    fn default() -> Self {
        Self {
            // textures: HashMap::with_capacity(Self::ASSETS_COUNT),
            // card_size: vec2(150., 180.),
            prefab_card: None,
            prefab_card_unit: None,
            prefab_card_spell: None,
            prefab_mana: None,
            // cards: HashMap::with_capacity(Self::START_CARD_COUNT),
            bd_cards: HashMap::with_capacity(5),
            players_node: HashMap::with_capacity(4),
            // cards_view: HashMap::with_capacity(4),
            config: Config::default(),
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
// pub struct CardStatsView {
//     name: RefLabel,
//     cost: Vec<ManaView>,
//     stats: CardTypeView,
// }
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
#[derive(Default)]
pub struct Config {
    pub screen_size: Vec2,
    pub screen_width: f32,
    pub screen_height: f32,
    pub screen_rect: Rect,
    pub card_indent: Vec2,
    pub card_size: Vec2,
}
impl Config {
    pub fn new(owner: &Node, card_size: Vec2, card_indent: Vec2) -> Self {
        let screen_size = owner
            .cast::<CanvasItem>()
            .map(|node| node.get_viewport_rect())
            .map(|viewport| {
                godot_print!(
                    "_{}-{}_ is screen pos! //// _{}-{}_ is screen size!",
                    viewport.position.x,
                    viewport.position.y,
                    viewport.size.x,
                    viewport.size.y,
                );
                viewport.size
            })
            .unwrap();
        Self {
            screen_rect: Rect::new(0., 0., screen_size.x, screen_size.y),
            screen_size,
            card_size,
            screen_width: screen_size.x,
            screen_height: screen_size.y,
            card_indent,
        }
    }
    pub fn is_up_side(&self, mouse_y: f32) -> bool {
        self.screen_size.y > mouse_y
    }
}

pub struct Player {
    tabel: Vec<CardId>,
    hand: Vec<CardId>,
    equipment: Equipment,
    character: Character,
    factories: Factories,
    deck: Deck,
}
pub struct Equipment {
    count: i64,
    items: Vec<i64>,
}
pub struct Deck {
    card_count: usize,
    dead_deck_count: i64,
    dead_deck: Vec<i64>,
}
pub struct Character {}
pub struct Factories {
    builds: Vec<i64>,
}
