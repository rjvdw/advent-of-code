use crate::cave::grid::Grid;

pub(in crate::cave) struct Gridterator {
    at: (usize, usize),
    width: usize,
    height: usize,
}

impl Gridterator {
    pub(in crate::cave) fn new(grid: &Grid) -> Gridterator {
        Gridterator {
            at: (0, 0),
            width: grid.width,
            height: grid.height,
        }
    }
}

impl Iterator for Gridterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.at;

        if y == self.height {
            None
        } else {
            self.at.0 += 1;
            if self.at.0 == self.width {
                self.at.0 = 0;
                self.at.1 += 1;
            }
            Some((x, y))
        }
    }
}
