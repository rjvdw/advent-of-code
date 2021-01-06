/// These are the factions that are battling in the cave.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Faction {
    Elf,
    Goblin,
}

impl Faction {
    /// Get the plural form of the faction.
    pub fn plural(&self) -> &str {
        match self {
            Faction::Elf => "elves",
            Faction::Goblin => "goblins",
        }
    }
}
