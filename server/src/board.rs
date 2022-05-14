use crate::*;
// use rand::Rng;

pub struct Board {
    queue_command: Vec<Event>, //Vec<Command>,
    history: Vec<Event>,       //Vec<Command>,
    // pub players: Vec<Player>,
    pub players: Vec<Player>,
    // player1: Player,
    // player2: Player,
}
impl Board {
    pub const NEEDED_FOR_MATCH: usize = 2;
    pub fn is_full(&self, len: usize) -> bool {
        Self::NEEDED_FOR_MATCH == len
    }
    pub fn is_ready(&self) -> bool {
        self.players.len() == 2
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            queue_command: Vec::with_capacity(10),
            history: Vec::with_capacity(40),
            players: Vec::with_capacity(2),
            // player1,
            // player2,
        }
    }
}
pub struct Player {
    tabel: Vec<CardId>, //Vec<Card>, //Tabel,  VecDeque
    hand: Vec<CardId>,  //Hand,
    // deck: Deck,
    // items: Items,
    // builds: Builds,
    // character: Character,
    //avatar
    healty: u64,

    // data: PlayerData,
    //
    pub player_handler: PlayerDataHandler,
    pub endpoint: Endpoint,
}
impl Player {
    pub fn new(endpoint: Endpoint, player_handler: PlayerDataHandler) -> Self {
        Self {
            tabel: Vec::with_capacity(8),
            hand: Vec::with_capacity(8),
            healty: 10,
            // data: PlayerData::default(),
            player_handler,
            endpoint,
        }
    }

    //TODO: send this hash card to dead_deck
    pub fn get_random_card_hash(&mut self) -> HashCard {
        // self.player_handler
        //     .data
        //     .vec_card
        //     .get(rand::thread_rng().gen_range(0..=self.player_handler.data.vec_card.len()))
        //     .unwrap()
        //     .clone()
        self.player_handler.data.vec_card.pop().unwrap() //shake deck
    }
    pub fn add_card_hand(&mut self, card_id: CardId) {
        self.hand.push(card_id);
    }
}
pub struct CardsMap {
    id_counter: CardId,
    map: HashMap<CardId, Card>,
    bd_cards: HashMap<HashCard, CardStats>,
}
impl CardsMap {
    pub fn new() -> Self {
        Self {
            id_counter: CardId::default(),
            map: HashMap::with_capacity(40),
            bd_cards: CardStatsBuilder::new_pool().into_iter().collect(),
        }
    }
    pub fn add_card(&mut self, hash_card: HashCard) -> CardId {
        let card_id = self.id_counter + 1;
        self.map.insert(
            card_id,
            Card {
                id: card_id,
                stats: Some(self.bd_cards.get_mut(&hash_card).unwrap().clone()),
            },
        );
        self.id_counter = card_id;
        card_id
    }
    pub fn get_bd(&self) -> Vec<(HashCard, CardStats)> {
        self.bd_cards.clone().into_iter().collect()
    }
}
#[derive(Default)]
pub struct PlayerSpawner {
    id_counter: PlayerId,
    players: HashMap<PlayerId, Player>,
}
impl PlayerSpawner {
    pub fn new_id(&mut self) -> PlayerId {
        self.id_counter += 1;
        self.id_counter
    }
}

// #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
// pub struct PlayerData {
//     pub name: String,
//     pub character: String,
//     pub vec_card: Vec<String>,
//     // pub player_type: PlayerType,
// }

// #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
// pub struct PlayerDataHandler {
//     pub avatar: String,
//     pub deck_name: String,
//     pub items_name: String,
//     pub builds_name: String,
//     pub character_name: String,
//     pub data: PlayerData,
// }
