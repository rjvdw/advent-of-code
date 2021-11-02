#[derive(Debug, Clone)]
pub struct Effect {
    pub(in crate::game_objects) turns: usize,
    pub(in crate::game_objects) armor: u32,
    pub(in crate::game_objects) damage: u32,
    pub(in crate::game_objects) mana_recovery: u32,
}
