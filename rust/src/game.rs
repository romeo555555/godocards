use crate::component::*;
use crate::input::*;
use crate::network::*;
use crate::resources::*;
use crate::system::*;
use crate::utils::*;
use bevy_ecs::prelude::*;
use gdnative::api::World as GDWorld;
// use gdnative::api::*;
use gdnative::prelude::*;
// enum State {
//     Auth,
//     Main,
//     Match,
//     Collections,
//     Exit,
// }

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct Game {
    world: World,
    resources: Resources,
    //selected_card
    //network
    game_match: Option<Match>,
}

#[methods]
impl Game {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Game builder is registered!");
    }
    fn new(_owner: &Node) -> Self {
        godot_print!("Game is created!");
        Self {
            world: World::new(),
            resources: Resources::default(),
            game_match: None,
        }
    }
    #[export]
    unsafe fn _ready(&mut self, owner: &Node) {
        logger::init(logger::Level::Info, logger::Output::File("log.txt")); //logger::Output::Stdout);
        log::info!("Closing server");
        self.resources.load_prefabs_and_config(owner);
        // self.world.spawn().insert_bundle(PlayerBundle::new(owner));
        // self.world.spawn().insert_bundle(PlayerBundle::new(owner));
        // self.world.spawn().insert_bundle(PlayerBundle::new(owner));
        // self.world.spawn().insert_bundle(PlayerBundle::new(owner));
        let entities = self
            .world
            .spawn_batch(vec![
                CardBundle::new(owner, &mut self.resources, 0),
                CardBundle::new(owner, &mut self.resources, 1),
                CardBundle::new(owner, &mut self.resources, 2),
            ])
            .collect::<Vec<Entity>>();
        godot_print!("Game is ready!");
    }
    #[export]
    unsafe fn _process(&mut self, owner: &Node, delta: f64) {
        // let entities = self
        //     .world
        //     .spawn_batch(vec![(LineT::A), (LineT::A), (LineT::B)])
        //     .collect::<Vec<Entity>>();
        // self.world.spawn().insert(LineT::A);
        // self.world.spawn().insert(LineT::A);
        // self.world.spawn().insert(LineT::B);

        // let mut query = self.world.query::<&LineT>();
        // godot_print!("{}", query.iter(&self.world).len());

        // for (mut position, velocity) in query.iter_mut(&mut world) {
        //     position.x += velocity.x;
        //     position.y += velocity.y;
        // }
        if let Some(ref mut game_match) = self.game_match {
            // game_match.draw(owner, &mut self.resources);
            // game_match.input(owner, &mut self.resources);
            // game_match.event(owner, &mut self.resources);
            log::info!("Cococclosing server");
        }
    }
    //Button does'nt work witch touuch
    #[export]
    fn _on_Match_pressed(&mut self, owner: &Node) {
        self.game_match = Some(Match::new(
            owner,
            &mut self.resources,
            PlayerDataHandler {
                id: PlayerId::default(),
                character_name: "avatarmini1".to_owned(),
                deck_name: "deck".to_owned(),
                items_name: "items".to_owned(),
                builds_name: "builds".to_owned(),
                avatar: "avatar".to_owned(),
                data: PlayerData {
                    name: "afkdsfv".to_owned(),
                    vec_card: vec![
                        "unit1".to_owned(),
                        "unit2".to_owned(),
                        "unit3".to_owned(),
                        "wizard".to_owned(),
                    ],
                    character: "avatarmini1".to_owned(),
                },
            },
        ));
    }
    #[export]
    fn _on_Collections_pressed(&mut self, _owner: &Node) {}
    #[export]
    fn _on_Exit_pressed(&mut self, _owner: &Node) {}
}

pub struct Match {
    history: Vec<Message>,
    network: Network,
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
            history: Vec::with_capacity(100),
            network,
        }
        // Self {
        //     selecting_card: SelectingCard::new(),
        //     players,
        //     line_for_update: None,
        // }
    }
}
// fn init(node: Node, world: &mut World) {
//     world.spawn().insert_bundle(PlayerBundle::new());
// }
