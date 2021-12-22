use std::collections::HashSet;

use crate::bounds::Bounds;
use crate::image::{Image, DARK_REGION, LIGHT_REGION};
use crate::point::Point;

pub(crate) trait SingleThreadedNext {
    /// Advances one generation.
    fn next(self) -> Self;
}

impl SingleThreadedNext for Image {
    fn next(self) -> Self {
        let mut lit = HashSet::new();
        let mut bounds = Bounds::default();
        let default_state = if self.default_state {
            self.iea[LIGHT_REGION]
        } else {
            self.iea[DARK_REGION]
        };

        for (row, col) in self.bounds.stretched(1).iter_row_col() {
            let iea_index = self.get_iea_index(row, col);
            if self.iea[iea_index] {
                lit.insert(Point::new(row, col));
                bounds.update_with(row, col);
            }
        }

        Image {
            iea: self.iea,
            lit,
            bounds,
            default_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared_test::test_image;

    use super::*;

    #[test]
    fn test_count_lit_pixels() {
        let image = test_image();
        assert_eq!(image.count_lit_pixels(), Ok(10));
        let image = SingleThreadedNext::next(image);
        assert_eq!(image.count_lit_pixels(), Ok(24));
        let image = SingleThreadedNext::next(image);
        assert_eq!(image.count_lit_pixels(), Ok(35));
    }
}
