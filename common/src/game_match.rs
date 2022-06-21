use crate::*;
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct MatchInfo {
    pub client_id: PlayerId,
    pub players_state: HashMap<PlayerId, PlayerState>,
    pub players_data: HashMap<PlayerId, PlayerData>,
    pub cards: HashMap<CardId, Option<CardState>>,
    pub bd_cards: HashMap<HashCard, CardState>,
    // pub opp_start_cards: HashMap<PlayerId, Vec<CardId>>,
    // pub start_cards: Vec<(CardId, HashCard)>,
}

// #[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
// pub enum MatchType {
//     Default,
//     // TwoFaces,
//     Match2x2,
// }
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum MatchState {
    None,
    StartGame,
    BeforeStep(u64),
    PlayerStep(u64),
    AfterStep(u64),
    EndGame,
}
