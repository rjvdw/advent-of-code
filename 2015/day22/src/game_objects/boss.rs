use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

use crate::game_objects::player::Player;

#[derive(Debug, Clone)]
pub struct Boss {
    hit_points: u32,
    damage: u32,
}

impl Boss {
    pub fn take_damage(&self, damage: u32) -> (Boss, bool) {
        let mut boss = self.clone();
        if damage > boss.hit_points {
            boss.hit_points = 0;
        } else {
            boss.hit_points -= damage;
        }
        let is_defeated = boss.hit_points == 0;
        (boss, is_defeated)
    }

    pub fn attack(&self, player: &Player) -> (Player, bool) {
        let mut player = player.clone();
        let damage = if self.damage <= player.armor {
            1
        } else {
            self.damage - player.armor
        };
        if damage > player.hit_points {
            player.hit_points = 0;
        } else {
            player.hit_points -= damage;
        }
        let is_defeated = player.hit_points == 0;
        (player, is_defeated)
    }
}

impl MultilineFromStr for Boss {
    type Err = ParseError;

    fn new() -> Self {
        Boss {
            hit_points: 0,
            damage: 0,
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if let Some(v) = line.strip_prefix("Hit Points: ") {
            self.hit_points = v.parse()?;
            Ok(())
        } else if let Some(v) = line.strip_prefix("Damage: ") {
            self.damage = v.parse()?;
            Ok(())
        } else {
            err_parse_error!("Invalid input line: {}", line)
        }
    }
}
