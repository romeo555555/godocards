use crate::*;
use rand::Rng;

// pub struct Player {
//     tabel: Vec<CardId>, //Vec<Card>, //Tabel,  VecDeque
//     hand: Vec<CardId>,  //Hand,
//     // deck: Deck,
//     // items: Items,
//     // builds: Builds,
//     // character: Character,
//     //avatar
//     healty: u64,

//     // data: PlayerData,
//     //
//     pub player_handler: PlayerDataHandler,
//     pub endpoint: Endpoint,
// }
// impl Player {
//     pub fn new(endpoint: Endpoint, player_handler: PlayerDataHandler) -> Self {
//         Self {
//             tabel: Vec::with_capacity(8),
//             hand: Vec::with_capacity(8),
//             healty: 10,
//             // data: PlayerData::default(),
//             player_handler,
//             endpoint,
//         }
//     }

//     //TODO: send this hash card to dead_deck
//     pub fn get_random_card_hash(&mut self) -> HashCard {
//         // self.player_handler
//         //     .data
//         //     .vec_card
//         //     .get(rand::thread_rng().gen_range(0..=self.player_handler.data.vec_card.len()))
//         //     .unwrap()
//         //     .clone()
//         // self.player_handler.data.vec_card.pop().unwrap() //shake deck
//         self.player_handler
//             .data
//             .vec_card
//             .get(rand::thread_rng().gen_range(0..self.player_handler.data.vec_card.len()))
//             .unwrap()
//             .clone() //shake deck
//     }
//     pub fn add_card_hand(&mut self, card_id: CardId) {
//         self.hand.push(card_id);
//     }
// }
