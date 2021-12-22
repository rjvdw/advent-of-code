use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use crate::bounds::Bounds;
use crate::image::{Image, DARK_REGION, LIGHT_REGION};
use crate::mt::divide::{Dividable, MAX_REGION_SIZE};
use crate::mt::shared_state::SharedState;
use crate::point::Point;
use crate::SingleThreadedNext;

pub(crate) trait MultiThreadedNext {
    /// Advances one generation.
    fn next(self) -> Self;
}

impl MultiThreadedNext for Image {
    fn next(self) -> Self {
        if self.bounds.size() <= MAX_REGION_SIZE {
            // sufficiently small that we don't need to bother with multithreading
            return SingleThreadedNext::next(self);
        }

        let shared_self = Arc::new(RwLock::new(self));
        let image = shared_self.read().unwrap();
        let shared_state = Arc::new(Mutex::new(SharedState {
            lit: HashSet::new(),
            bounds: Bounds::default(),
        }));
        let default_state = if image.default_state {
            image.iea[LIGHT_REGION]
        } else {
            image.iea[DARK_REGION]
        };

        let mut handles = vec![];
        for region in image.bounds.stretched(1).divide() {
            let shared_self = Arc::clone(&shared_self);
            let shared_state = Arc::clone(&shared_state);

            let mut lit = HashSet::<Point>::new();
            let mut bounds = Bounds::default();

            let handle = thread::spawn(move || {
                let image = shared_self.read().unwrap();
                for (row, col) in region.iter_row_col() {
                    let iea_index = image.get_iea_index(row, col);
                    if image.iea[iea_index] {
                        lit.insert(Point::new(row, col));
                        bounds.update_with(row, col);
                    }
                }

                if !lit.is_empty() {
                    let mut state = shared_state.lock().unwrap();
                    state.lit.extend(&lit);
                    state.bounds.join_with(&bounds);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let state = &*shared_state.lock().unwrap();
        Image {
            iea: image.iea,
            lit: state.lit.clone(),
            bounds: state.bounds,
            default_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared_test::test_image;

    use super::*;

    #[test]
    fn test_count_lit_pixels_mt() {
        let image = test_image();
        assert_eq!(image.count_lit_pixels(), Ok(10));
        let image = MultiThreadedNext::next(image);
        assert_eq!(image.count_lit_pixels(), Ok(24));
        let image = MultiThreadedNext::next(image);
        assert_eq!(image.count_lit_pixels(), Ok(35));
    }
}
