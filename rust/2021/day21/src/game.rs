use crate::Player;

#[derive(Copy, Clone)]
pub struct Game {
    players: [Player; 2],
    turn: usize,
    target_score: u64,
}

impl Game {
    pub fn new(players: &[Player], target_score: u64) -> Game {
        Game {
            players: [players[0], players[1]],
            turn: 0,
            target_score,
        }
    }

    pub fn play(&mut self, roll: u64) -> Option<(usize, [Player; 2])> {
        let player = self.players[self.turn].mv(roll);
        self.players[self.turn] = player;
        if player.has_won(self.target_score) {
            Some((self.turn, self.players))
        } else {
            self.turn = self.turn ^ 1;
            None
        }
    }
}
