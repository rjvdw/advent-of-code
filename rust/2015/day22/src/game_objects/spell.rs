use crate::game_objects::effect::Effect;

pub struct Spell<'a> {
    pub(in crate::game_objects) name: &'a str,
    pub(in crate::game_objects) mana: u32,
    pub(in crate::game_objects) damage: u32,
    pub(in crate::game_objects) heals: u32,
    pub(in crate::game_objects) effect: Option<Effect>,
}

impl Spell<'static> {
    pub const SPELLS: [Spell<'static>; 5] = [
        Spell {
            name: "Magic Missile",
            mana: 53,
            damage: 4,
            heals: 0,
            effect: None,
        },
        Spell {
            name: "Drain",
            mana: 73,
            damage: 2,
            heals: 2,
            effect: None,
        },
        Spell {
            name: "Shield",
            mana: 113,
            damage: 0,
            heals: 0,
            effect: Some(Effect {
                turns: 6,
                armor: 7,
                damage: 0,
                mana_recovery: 0,
            }),
        },
        Spell {
            name: "Poison",
            mana: 173,
            damage: 0,
            heals: 0,
            effect: Some(Effect {
                turns: 6,
                armor: 0,
                damage: 3,
                mana_recovery: 0,
            }),
        },
        Spell {
            name: "Recharge",
            mana: 229,
            damage: 0,
            heals: 0,
            effect: Some(Effect {
                turns: 5,
                armor: 0,
                damage: 0,
                mana_recovery: 101,
            }),
        },
    ];

    pub fn get_cost(&self) -> u32 {
        self.mana
    }
}
