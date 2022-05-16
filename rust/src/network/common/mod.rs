use crate::*;
use nanoserde::{DeBin, DeJson, SerBin, SerJson};
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
pub struct Mana {
    pub(crate) count: u64,
    pub(crate) mana_form: ManaForm,
}
#[derive(Clone, Debug, DeJson, SerJson, DeBin, SerBin, PartialEq)]
pub enum ManaForm {
    Once(ManaColor),
    Two([ManaColor; 2]),
    Three([ManaColor; 3]),
    Four([ManaColor; 4]),
    Uncolor,
}
// struct ManaTuple()

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
    pub(crate) brute_force: u64,
    pub(crate) intelligence: u64,
    pub(crate) magical_potential: u64,
    pub(crate) adaptability: u64,
    pub(crate) mastery: u64,

    // attack_type: AttackType, DamageType,
    pub(crate) attack: u64,
    pub(crate) healty: u64,
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
}
