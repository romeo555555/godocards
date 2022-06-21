use crate::*;
//TODO: delete this from common and switch to static data from bd
pub struct CardStateBuilder {}
impl CardStateBuilder {
    pub fn build(hash: HashCard, cost: Vec<Mana>, card_type: CardType) -> CardState {
        CardState {
            name: hash.clone(),
            description: hash.clone(),
            hash,
            cost,
            card_type,
        }
    }
    pub fn new_pool() -> Vec<(HashCard, CardState)> {
        vec![
            (
                "unit1".to_owned(),
                CardStateBuilder::build(
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
                CardStateBuilder::build(
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
                CardStateBuilder::build(
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
                "unit4".to_owned(),
                CardStateBuilder::build(
                    "unit4".to_owned(),
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
                "unit5".to_owned(),
                CardStateBuilder::build(
                    "unit5".to_owned(),
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
                "unit6".to_owned(),
                CardStateBuilder::build(
                    "unit6".to_owned(),
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
                "unit7".to_owned(),
                CardStateBuilder::build(
                    "unit7".to_owned(),
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
                "unit8".to_owned(),
                CardStateBuilder::build(
                    "unit8".to_owned(),
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
                "wizard1".to_owned(),
                CardStateBuilder::build(
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
            (
                "wizard2".to_owned(),
                CardStateBuilder::build(
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
