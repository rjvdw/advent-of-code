use crate::tile::Tile;
use crate::Cave;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct State {
    pub(crate) positions: Vec<(usize, usize)>,
    pub(crate) keys: Vec<Vec<char>>,
    pub(crate) keys_per_section: Vec<usize>,
}

impl State {
    pub(crate) fn initial_state(cave: &Cave) -> State {
        let mut state = State {
            positions: vec![],
            keys: vec![],
            keys_per_section: vec![0; 4],
        };
        for y in 0..cave.layout.rows() {
            for x in 0..cave.layout.cols() {
                if matches!(cave.layout[y][x], Tile::Entrance) {
                    state.positions.push((x, y));
                    state.keys.push(vec![]);
                }

                if matches!(cave.layout[y][x], Tile::Key(_)) {
                    let idx;
                    if y < cave.layout.rows() / 2 {
                        if x < cave.layout.cols() / 2 {
                            idx = 0;
                        } else {
                            idx = 1;
                        }
                    } else if x < cave.layout.cols() / 2 {
                        idx = 2;
                    } else {
                        idx = 3;
                    }
                    state.keys_per_section[idx] += 1;
                }
            }
        }
        if state.positions.len() == 1 {
            state.keys_per_section[0] +=
                state.keys_per_section[1] + state.keys_per_section[2] + state.keys_per_section[3];
        }
        state
    }

    pub(crate) fn get_neighbours(&self) -> Vec<((usize, usize), usize, State)> {
        // assumption: We are surrounded by walls so will never reach (0, _) or (_, 0)
        // therefore, we do not need bound checks

        let mut neighbours = vec![];
        for (idx, &(x, y)) in self.positions.iter().enumerate() {
            if self.keys_remaining(idx) {
                let mut neighbour = self.clone();
                neighbour.positions[idx] = (x, y - 1);
                neighbours.push(((x, y - 1), idx, neighbour));

                let mut neighbour = self.clone();
                neighbour.positions[idx] = (x, y + 1);
                neighbours.push(((x, y + 1), idx, neighbour));

                let mut neighbour = self.clone();
                neighbour.positions[idx] = (x - 1, y);
                neighbours.push(((x - 1, y), idx, neighbour));

                let mut neighbour = self.clone();
                neighbour.positions[idx] = (x + 1, y);
                neighbours.push(((x + 1, y), idx, neighbour));
            }
        }
        neighbours
    }

    pub(crate) fn add_key(&mut self, key: char, idx: usize) {
        self.keys[idx].push(key);
        self.keys[idx].sort_unstable();
    }

    pub(crate) fn has_key(&self, key: char) -> bool {
        self.keys.iter().any(|keys| keys.contains(&key))
    }

    pub(crate) fn nr_keys(&self) -> usize {
        self.keys.iter().map(|keys| keys.len()).sum::<usize>()
    }

    pub(crate) fn keys_remaining(&self, idx: usize) -> bool {
        self.keys_per_section[idx] > self.keys[idx].len()
    }
}
