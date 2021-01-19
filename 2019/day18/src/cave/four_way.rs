use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use rdcl_aoc_helpers::error::ParseError;

use crate::cave::four_way::state::{HashedState, ScoredState, State};
use crate::cave::grid::Grid;
use crate::cave::Cave;

mod state;

#[derive(Debug, Clone)]
pub struct FourWayCave {
    layout: Grid,
    entrances: [(usize, usize); 4],
}

impl Cave for FourWayCave {
    fn find_shortest_path(&self) -> Option<usize> {
        let mut layout = self.layout.clone();
        let initial_state = State {
            positions: self.entrances,
            keys: vec![],
            distance: 0,
        };
        let mut exploring: BinaryHeap<ScoredState> = BinaryHeap::new();
        exploring.push(ScoredState(initial_state));

        let mut seen: HashSet<HashedState> = HashSet::new();

        let mut best: Option<usize> = None;

        while let Some(state) = exploring.pop() {
            let state = state.0;
            if let Some(distance) = best {
                if state.distance > distance {
                    return best;
                }
            }
            let hashed_state = HashedState(state.clone());
            if seen.contains(&hashed_state) {
                continue;
            }
            seen.insert(hashed_state);
            for i in 0..4 {
                for reachable in layout.get_reachable(state.positions[i], &state.keys) {
                    let mut next_state = state.clone();
                    next_state.positions[i] = reachable.position;
                    next_state.distance += reachable.distance;
                    next_state.keys.push(reachable.key);
                    next_state.keys.sort_unstable();
                    if next_state.keys.len() == layout.keys.len() {
                        match best {
                            Some(distance) => {
                                best = Some(distance.min(next_state.distance));
                            }
                            None => {
                                best = Some(next_state.distance);
                            }
                        }
                    } else {
                        exploring.push(ScoredState(next_state));
                    }
                }
            }
        }

        best
    }

    fn parse<R: Read>(r: R) -> Result<Self, ParseError> {
        let layout = Grid::parse(r)?;
        assert_eq!(layout.entrances.len(), 4);
        let entrances = [
            layout.entrances[0],
            layout.entrances[1],
            layout.entrances[2],
            layout.entrances[3],
        ];
        Ok(FourWayCave { layout, entrances })
    }
}

impl Display for FourWayCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layout)
    }
}
