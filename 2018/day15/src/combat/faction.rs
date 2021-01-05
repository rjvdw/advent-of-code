#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Faction {
    Elf,
    Goblin,
}

impl Faction {
    pub fn plural(&self) -> &str {
        match self {
            Faction::Elf => "elves",
            Faction::Goblin => "goblins",
        }
    }
}
