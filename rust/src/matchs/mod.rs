use crate::*;
use gdnative::api::Node;
pub use match1x1::*;
pub use match2x2::*;
pub use match_two_faces::*;
mod match1x1;
mod match2x2;
mod match_two_faces;

pub enum Match {
    Match1x1(Match1x1),
    // Match2x2(Match2x2),
    // MatchTwoFaces(MatchTwoFaces),
}
impl Match {
    pub fn input(&mut self, owner: &Node, ctx: &mut Resources) {
        match self {
            Match::Match1x1(game_match) => {
                game_match.input(owner, ctx);
            }
        }
    }
    pub fn event(&mut self, owner: &Node, ctx: &mut Resources) {
        match self {
            Match::Match1x1(game_match) => {
                game_match.event(owner, ctx);
            }
        }
    }
}
