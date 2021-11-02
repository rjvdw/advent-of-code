use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use rdcl_aoc_helpers::error::ParseError;

use crate::cave::grid::Grid;
use crate::cave::simple::state::{HashedState, ScoredState, State};
use crate::cave::Cave;

mod state;

#[derive(Debug, Clone)]
pub struct SimpleCave {
    layout: Grid,
    entrance: (usize, usize),
}

impl Cave for SimpleCave {
    fn find_shortest_path(&self) -> Option<usize> {
        let mut layout = self.layout.clone();
        let initial_state = State {
            position: self.entrance,
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
            for reachable in layout.get_reachable(state.position, &state.keys) {
                let mut next_state = State {
                    position: reachable.position,
                    keys: state.keys.clone(),
                    distance: state.distance + reachable.distance,
                };
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

        best
    }

    fn parse<R: Read>(r: R) -> Result<SimpleCave, ParseError> {
        let layout = Grid::parse(r)?;
        assert_eq!(layout.entrances.len(), 1);
        let entrance = layout.entrances[0];
        Ok(SimpleCave { layout, entrance })
    }
}

impl Display for SimpleCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layout)
    }
}
