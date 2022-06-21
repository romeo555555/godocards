use crate::*;

pub type CardId = u64;
pub type HashCard = String;

#[derive(Clone, Debug, DeJson, SerJson, PartialEq, DeBin, SerBin)]
pub struct CardState {
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
}
