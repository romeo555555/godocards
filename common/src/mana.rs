use crate::*;
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
