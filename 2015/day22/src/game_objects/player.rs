use std::collections::HashMap;

use crate::game_objects::effect::Effect;
use crate::game_objects::spell::Spell;

#[derive(Debug, Clone)]
pub struct Player {
    pub(in crate::game_objects) hit_points: u32,
    pub(in crate::game_objects) mana: u32,
    pub(in crate::game_objects) armor: u32,
    pub(in crate::game_objects) effects: HashMap<String, Effect>,
}

impl Player {
    pub fn new(hit_points: u32, mana: u32) -> Player {
        Player {
            hit_points,
            mana,
            armor: 0,
            effects: HashMap::new(),
        }
    }

    pub fn apply_effects(&self) -> (Player, u32) {
        let mut player = self.clone();
        player.armor = 0;
        let mut damage = 0;
        let mut effects = HashMap::new();

        for (name, effect) in &self.effects {
            damage += effect.damage;
            player.armor += effect.armor;
            player.mana += effect.mana_recovery;
            if effect.turns > 1 {
                let mut effect = effect.clone();
                effect.turns -= 1;
                effects.insert(name.to_string(), effect);
            }
        }

        player.effects = effects;

        (player, damage)
    }

    pub fn cast(&self, spell: &Spell) -> Option<(Player, u32)> {
        if self.mana >= spell.mana
            && (spell.effect.is_none() || !self.effects.contains_key(spell.name))
        {
            let mut effects = self.effects.clone();
            if let Some(effect) = &spell.effect {
                effects.insert(spell.name.to_string(), effect.clone());
            }
            Some((
                Player {
                    hit_points: self.hit_points + spell.heals,
                    mana: self.mana - spell.mana,
                    armor: self.armor,
                    effects,
                },
                spell.damage,
            ))
        } else {
            None
        }
    }
    pub fn take_damage(&self, damage: u32) -> (Player, bool) {
        let mut player = self.clone();
        if damage > player.hit_points {
            player.hit_points = 0;
        } else {
            player.hit_points -= damage;
        }
        let is_defeated = player.hit_points == 0;
        (player, is_defeated)
    }
}
