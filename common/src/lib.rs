use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use rand::Rng;
use std::collections::HashMap;

//It's the same file with in server
//Do this in own crate lib
pub type CardId = u64;
pub type HashCard = String;

pub type PlayerId = u64; //String;

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct MatchInfo {
    pub client_id: PlayerId,
    pub players: HashMap<PlayerId, PlayerDataHandler>,
    pub opp_start_cards: HashMap<PlayerId, Vec<CardId>>,
    pub start_cards: Vec<(CardId, HashCard)>,
    pub bd_cards: Vec<(HashCard, CardStats)>,
}

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum MatchType {
    Default,
    TwoFaces,
    Match2x2,
}

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct Message {
    pub player_id: PlayerId,
    pub event: Event,
}
impl Message {
    pub fn build(player_id: PlayerId, event: Event) -> Self {
        Self { player_id, event }
    }
}
//ManaUpdate it's need to change
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum Event {
    ChangeState(State),
    TakeCard(CardId),
    FlipCard(CardId, HashCard),
    RemoveCard(CardId),
    CastCardOnTabel(CardId),
    BackCardOnHand(CardId),
    ManaUpdate(u64, ManaColor),
    //Attack
    //CaAddstCardSpale
    //AddCardOpponent
    //AddUnitOpponent
    //AddUnit
}
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum State {
    None,
    StartGame,
    BeforeStep(u64),
    PlayerStep(u64),
    AfterStep(u64),
    EndGame,
}

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct PlayerDataHandler {
    pub id: PlayerId,
    pub avatar: String,
    pub deck_name: String,
    pub items_name: String,
    pub builds_name: String,
    pub character_name: String,
    pub data: PlayerData,
}

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct PlayerData {
    pub name: String,
    pub character: String,
    pub vec_card: Vec<HashCard>,
}

#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Mana {
    pub count: u64,
    pub mana_form: ManaForm,
}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub enum ManaForm {
    Once(ManaColor),
    Two([ManaColor; 2]),
    Three([ManaColor; 3]),
    Four([ManaColor; 4]),
    Uncolor,
}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub enum ManaColor {
    Red,
    Blue,
    Green,
    Black,
    White,
}

#[derive(Clone, Debug, DeJson, SerJson, PartialEq, DeBin, SerBin)]
pub struct CardStats {
    pub name: String,
    pub hash: HashCard,
    pub cost: Vec<Mana>,
    pub card_type: CardType,
    pub description: String,
}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Unit {
    pub brute_force: u64,
    pub intelligence: u64,
    pub magical_potential: u64,
    pub adaptability: u64,
    pub mastery: u64,

    // attack_type: AttackType, DamageType,
    pub attack: u64,
    pub healty: u64,
}
// impl IntoIterator for Unit {
//     type Item = u64;
//     type IntoIter = std::array::IntoIter<u64, 7>;

//     fn into_iter(self) -> Self::IntoIter {
//         std::array::IntoIter::new([
//             self.brute_force,
//             self.intelligence,
//             self.magical_potential,
//             self.adaptability,
//             self.mastery,
//             self.attack,
//             self.healty,
//         ])
//     }
// }
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Spell {
    // multiply_damage: u64, //type magic//tochnosty
}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Build {}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Item {}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Zone {}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub enum CardType {
    Unit(Unit),
    Spell(Spell),
    Build(Build),
    Item(Item),
    Zone(Zone),
} //TODO: delete this from common and switch to static data from bd
pub struct CardStatsBuilder {}
impl CardStatsBuilder {
    pub fn build(hash: HashCard, cost: Vec<Mana>, card_type: CardType) -> CardStats {
        CardStats {
            name: hash.clone(),
            description: hash.clone(),
            hash,
            cost,
            card_type,
        }
    }
    pub fn new_pool() -> Vec<(HashCard, CardStats)> {
        vec![
            (
                "unit1".to_owned(),
                CardStatsBuilder::build(
                    "unit1".to_owned(),
                    vec![Mana {
                        count: rand::thread_rng().gen_range(0..=9),
                        mana_form: ManaForm::Four([
                            ManaColor::Blue,
                            ManaColor::Green,
                            ManaColor::White,
                            ManaColor::Black,
                        ]),
                    }],
                    CardType::Unit(Unit {
                        brute_force: rand::thread_rng().gen_range(0..=9),
                        intelligence: rand::thread_rng().gen_range(0..=9),
                        magical_potential: rand::thread_rng().gen_range(0..=9),
                        adaptability: rand::thread_rng().gen_range(0..=9),
                        mastery: rand::thread_rng().gen_range(0..=9),

                        // attack_type: AttackType, DamageType,
                        attack: rand::thread_rng().gen_range(0..=9),
                        healty: rand::thread_rng().gen_range(0..=9),
                    }),
                ),
            ),
            (
                "unit2".to_owned(),
                CardStatsBuilder::build(
                    "unit2".to_owned(),
                    vec![
                        Mana {
                            count: rand::thread_rng().gen_range(0..=9),
                            mana_form: ManaForm::Two([ManaColor::Blue, ManaColor::Green]),
                        },
                        Mana {
                            count: rand::thread_rng().gen_range(0..=9),
                            mana_form: ManaForm::Once(ManaColor::Blue),
                        },
                    ],
                    CardType::Unit(Unit {
                        brute_force: rand::thread_rng().gen_range(0..=9),
                        intelligence: rand::thread_rng().gen_range(0..=9),
                        magical_potential: rand::thread_rng().gen_range(0..=9),
                        adaptability: rand::thread_rng().gen_range(0..=9),
                        mastery: rand::thread_rng().gen_range(0..=9),

                        // attack_type: AttackType, DamageType,
                        attack: rand::thread_rng().gen_range(0..=9),
                        healty: rand::thread_rng().gen_range(0..=9),
                    }),
                ),
            ),
            (
                "unit3".to_owned(),
                CardStatsBuilder::build(
                    "unit3".to_owned(),
                    vec![
                        Mana {
                            count: rand::thread_rng().gen_range(0..=9),
                            mana_form: ManaForm::Once(ManaColor::Red),
                        },
                        Mana {
                            count: rand::thread_rng().gen_range(0..=9),
                            mana_form: ManaForm::Once(ManaColor::Green),
                        },
                        Mana {
                            count: rand::thread_rng().gen_range(0..=9),
                            mana_form: ManaForm::Two([ManaColor::Blue, ManaColor::Green]),
                        },
                    ],
                    CardType::Unit(Unit {
                        brute_force: rand::thread_rng().gen_range(0..=9),
                        intelligence: rand::thread_rng().gen_range(0..=9),
                        magical_potential: rand::thread_rng().gen_range(0..=9),
                        adaptability: rand::thread_rng().gen_range(0..=9),
                        mastery: rand::thread_rng().gen_range(0..=9),

                        // attack_type: AttackType, DamageType,
                        attack: rand::thread_rng().gen_range(0..=9),
                        healty: rand::thread_rng().gen_range(0..=9),
                    }),
                ),
            ),
            (
                "wizard".to_owned(),
                CardStatsBuilder::build(
                    "wizard".to_owned(),
                    vec![Mana {
                        count: rand::thread_rng().gen_range(0..=9),
                        mana_form: ManaForm::Three([
                            ManaColor::Blue,
                            ManaColor::Green,
                            ManaColor::White,
                        ]),
                    }],
                    CardType::Spell(Spell {}),
                ),
            ),
        ]
    }
}
// pub enum PID {
//     Player1,
//     Opp1,
//     Opp2,
//     Friend,
// }
// pub struct Players {
//     player1: Player,
//     player2: Player,
//     player3: Option<Player>,
//     player4: Option<Player>,
// }
// impl Players {
//     pub fn new(client: Player, opp1: Player) -> Self {
//         Self {
//             client,
//             opp1,
//             opp2: None,
//             friend: None,
//         }
//     }
//     pub fn new2x2(client: Player, opp1: Player, opp2: Player, friend: Player) -> Self {
//         Self {
//             client,
//             opp1,
//             opp2: Some(opp2),
//             friend: Some(friend),
//         }
//     }
// }

// impl Index<PID> for Players {
//     type Output = gui::player::Player;

//     fn index(&self, pid: PID) -> &Self::Output {
//         match pid {
//             PID::Client => &self.client,
//             PID::Opp1 => &self.opp1,
//             PID::Opp2 => &self.opp2.unwrap(),
//             PID::Friend => &self.friend.unwrap(),
//         }
//     }
// }
// impl IndexMut<PID> for Players {
//     fn index_mut(&mut self, pid: PID) -> &mut Self::Output {
//         match pid {
//             PID::Client => &mut self.client,
//             PID::Opp1 => &mut self.opp1,
//             PID::Opp2 => &mut self.opp2.unwrap(),
//             PID::Friend => &mut self.friend.unwrap(),
//         }
//     }
// }
