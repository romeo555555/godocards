use crate::*;
use nanoserde::{DeJson, SerJson};
// use std::{cmp::Ordering, ops::Add};

#[derive(Debug)]
pub struct ManaPool {
    all_count: u64,
    red: u64,
    blue: u64,
    green: u64,
    black: u64,
    white: u64,

    cmp_all_count: u64,
}
impl Default for ManaPool {
    fn default() -> Self {
        Self {
            all_count: 100,
            red: 20,
            blue: 20,
            green: 20,
            black: 20,
            white: 20,
            cmp_all_count: 100,
        }
    }
}
impl ManaPool {
    // pub fn new(rect:Rect)->Self {
    //     Self{
    //         rect,
    //     }
    // }
    //TODO:query what type mana i want pay
    // pub fn try_pay(&mut self, mana_cost: &Vec<ManaForm>) -> bool {
    //     if self.check_cost(mana_cost) {
    //         self.pay(mana_cost);
    //         return true;
    //     }
    //     false
    // }
    // fn check_cost(&mut self, mana_cost: &Vec<ManaForm>) -> bool {
    //     self.cmp_all_count = self.all_count;
    //     for cost in mana_cost {
    //         if match cost {
    //             ManaForm::Once(mana) => self.is_not_enough(mana),
    //             ManaForm::Two(mana1, mana2) => {
    //                 self.is_not_enough(mana1) && self.is_not_enough(mana2)
    //             }
    //             ManaForm::Three(mana1, mana2, mana3) => {
    //                 self.is_not_enough(mana1)
    //                     && self.is_not_enough(mana2)
    //                     && self.is_not_enough(mana3)
    //             }
    //             ManaForm::Four(mana1, mana2, mana3, mana4) => {
    //                 self.is_not_enough(mana1)
    //                     && self.is_not_enough(mana2)
    //                     && self.is_not_enough(mana3)
    //                     && self.is_not_enough(mana4)
    //             }
    //             ManaForm::Uncolor(count) => self.cmp_all_count < *count,
    //         } {
    //             return false;
    //         }
    //     }
    //     true
    // }
    // fn is_not_enough(&mut self, other: &Mana) -> bool {
    //     let color_and_mana = self.get_color_and_mana_count(other);
    //     if *color_and_mana.0 < color_and_mana.1 {
    //         return true;
    //     }
    //     self.cmp_all_count -= color_and_mana.1;
    //     false
    // }
    // fn is_not_pay(&mut self, other: &Mana) -> bool {
    //     let color_and_mana = self.get_color_and_mana_count(other);
    //     let (result, complet) = color_and_mana.0.overflowing_sub(color_and_mana.1);
    //     if complet {
    //         return true;
    //     }
    //     *color_and_mana.0 = result;
    //     self.all_count -= color_and_mana.1;
    //     false
    // }
    // pub fn add_mana(&mut self, mana: Mana) {
    //     let color_and_mana = self.get_color_and_mana_count(&mana);
    //     *color_and_mana.0 += color_and_mana.1;
    //     self.all_count += color_and_mana.1;
    // }
    // fn get_color_and_mana_count(&mut self, other: &Mana) -> (&mut u64, u64) {
    //     match other {
    //         Mana::Red(count) => (&mut self.red, *count),
    //         Mana::Blue(count) => (&mut self.blue, *count),
    //         Mana::Green(count) => (&mut self.green, *count),
    //         Mana::Black(count) => (&mut self.black, *count),
    //         Mana::White(count) => (&mut self.white, *count),
    //     }
    // }
    // pub fn print(&self) -> String {
    //     " | ".to_string().to_owned()
    //         + &self.red.to_string().clone()
    //         + " \n "
    //         + &self.blue.to_string().clone()
    //         + " \n "
    //         + &self.green.to_string().clone()
    //         + " \n "
    //         + &self.black.to_string().clone()
    //         + " \n "
    //         + &self.white.to_string().clone()
    //         + &" | ".to_string().to_owned()
    // }
    // fn pay(&mut self, mana_cost: &Vec<ManaForm>) {
    //     for cost in mana_cost {
    //         match cost {
    //             ManaForm::Once(mana) => {
    //                 self.is_not_pay(mana);
    //             }
    //             ManaForm::Two(mana1, mana2) => {
    //                 if self.is_not_pay(mana1) {
    //                     self.is_not_pay(mana2);
    //                 }
    //             }
    //             ManaForm::Three(mana1, mana2, mana3) => {
    //                 if self.is_not_pay(mana1) {
    //                     if self.is_not_pay(mana2) {
    //                         self.is_not_pay(mana3);
    //                     }
    //                 }
    //             }
    //             ManaForm::Four(mana1, mana2, mana3, mana4) => {
    //                 if self.is_not_pay(mana1) {
    //                     if self.is_not_pay(mana2) {
    //                         if self.is_not_pay(mana3) {
    //                             self.is_not_pay(mana4);
    //                         }
    //                     }
    //                 }
    //             }
    //             ManaForm::Uncolor(count) => {
    //                 if self.is_not_pay(&Mana::Red(*count)) {
    //                     if self.is_not_pay(&Mana::Blue(*count)) {
    //                         if self.is_not_pay(&Mana::Green(*count)) {
    //                             if self.is_not_pay(&Mana::Black(*count)) {
    //                                 self.is_not_pay(&Mana::White(*count));
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

// #[derive(Clone, Debug, DeJson, SerJson)]
// pub enum Mana {
//     Red(u64),
//     Blue(u64),
//     Green(u64),
//     Black(u64),
//     White(u64),
// }

// #[derive(Clone, Debug, DeJson, SerJson)]
// pub enum ManaForm {
//     Once(Mana),
//     Two(Mana, Mana),
//     Three(Mana, Mana, Mana),
//     Four(Mana, Mana, Mana, Mana),
//     UnColor(u64),
// }
