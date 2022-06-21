use nanoserde::{DeBin, DeJson, SerBin, SerJson};
use rand::Rng;
use std::collections::HashMap;
pub mod card;
pub mod card_builder;
pub mod game_match;
pub mod mana;
pub mod player;
use card::*;
use game_match::*;
use mana::*;
use player::*;

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum ClientAction {
    EndStep,
    CastCardOnTabel(CardId),
    BackCardOnHand(CardId),
    ManaUpdate(u64, ManaColor),
    //Attack
    //CaAddstCardSpale
    //AddCardOpponent
    //AddUnitOpponent
    //AddUnit
    RemoveCard(CardId),
    FlipCard(CardId, HashCard),
    TakeCard,
}
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct ClientMessage {
    // pub player_id: PlayerId,
    pub action: ClientAction,
}
impl ClientMessage {
    pub fn build(action: ClientAction) -> Self {
        Self { action }
    }
}

#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub struct ServerMessage {
    pub player_id: PlayerId,
    pub action: ServerAction,
}
impl ServerMessage {
    pub fn build(player_id: PlayerId, action: ServerAction) -> Self {
        Self { player_id, action }
    }
}
//ManaUpdate it's need to change
#[derive(Debug, Clone, SerBin, DeBin, PartialEq)]
pub enum ServerAction {
    ChangeState(MatchState),
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
