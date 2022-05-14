use crate::*;
use nanoserde::{DeBin, DeJson, SerBin, SerJson};
// use rand::Rng;
use std::collections::HashMap;

//It's the same file with in server
//Do this in own crate lib
pub type PlayerId = u64; //String;
pub type CardId = u64;
pub type HashCard = String;

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
pub struct Query {
    match_type: MatchType,
    player_data: PlayerDataHandler,
}

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum Message {
    Message(Msg),
    MatchInfo(MatchInfo),
}
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct Msg {
    pub player_id: PlayerId,
    pub event: Event,
}
impl Msg {
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
    ManaUpdate(Mana),
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

// PlayerDataHandler {
//                 character_name: "avatarmini".to_owned(),
//                 deck_name: "deck".to_owned(),
//                 items_name: "items".to_owned(),
//                 builds_name: "builds".to_owned(),
//                 avatar: "avatar".to_owned(),
//                 data: PlayerData {
//                     name: "klvgjrv".to_owned(),
//                     vec_card: Vec::with_capacity(30),
//                     player_type: PlayerType::Remote,
//                     character: "avatarmini".to_owned(),
//                 },
//             },
//             PlayerDataHandler {
//                 character_name: "avatarmini1".to_owned(),
//                 deck_name: "deck".to_owned(),
//                 items_name: "items".to_owned(),
//                 builds_name: "builds".to_owned(),
//                 avatar: "avatar".to_owned(),
//                 data: PlayerData {
//                     name: "afkdsfv".to_owned(),
//                     vec_card: Vec::with_capacity(30),
//                     player_type: PlayerType::Client,
//                     character: "avatarmini1".to_owned(),
//                 },
//             },
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub enum Mana {
    Red(u64),
    Blue(u64),
    Green(u64),
    Black(u64),
    White(u64),
}

#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub enum ManaForm {
    Once(Mana),
    Two(Mana, Mana),
    Three(Mana, Mana, Mana),
    Four(Mana, Mana, Mana, Mana),
    UnColor(u64),
}

#[derive(Clone, Debug, DeJson, SerJson, PartialEq, DeBin, SerBin)]
pub struct CardStats {
    pub name: String,
    pub hash: HashCard,
    pub cost: Vec<ManaForm>,
    pub card_type: CardType,
    pub description: String,
}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub struct Unit {
    brute_force: u64,
    intelligence: u64,
    magical_potential: u64,
    adaptability: u64,
    mastery: u64,

    // attack_type: AttackType, DamageType,
    pub attack: u64,
    pub healty: u64,
}
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
}

//TODO: delete this from common and switch to static data from bd
pub struct CardStatsBuilder {}
impl CardStatsBuilder {
    pub fn new(hash: HashCard, cost: Vec<ManaForm>, card_type: CardType) -> CardStats {
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
                CardStatsBuilder::new(
                    "unit1".to_owned(),
                    vec![ManaForm::Once(Mana::Red(
                        rand::thread_rng().gen_range(0..=9),
                    ))],
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
                CardStatsBuilder::new(
                    "unit2".to_owned(),
                    vec![ManaForm::Two(
                        Mana::Red(rand::thread_rng().gen_range(0..=9)),
                        Mana::Blue(rand::thread_rng().gen_range(0..=9)),
                    )],
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
                CardStatsBuilder::new(
                    "unit3".to_owned(),
                    vec![ManaForm::Once(Mana::Black(
                        rand::thread_rng().gen_range(0..=9),
                    ))],
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
                CardStatsBuilder::new(
                    "wizard".to_owned(),
                    vec![ManaForm::Once(Mana::Black(
                        rand::thread_rng().gen_range(0..=9),
                    ))],
                    CardType::Spell(Spell {}),
                ),
            ),
        ]
    }
}
