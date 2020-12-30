use std::fmt;

pub struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height {
            pixels.push(vec![false; width]);
        }
        Screen { pixels }
    }

    pub fn turn_on(&mut self, row: usize, column: usize) {
        self.pixels[row][column] = true;
    }

    pub fn shift_row(&mut self, row: usize, by: usize) {
        let row = &mut self.pixels[row];
        let copy = row.clone();

        for (old_idx, pixel) in copy.iter().enumerate() {
            let new_idx = (old_idx + by) % row.len();
            row[new_idx] = *pixel;
        }
    }

    pub fn shift_column(&mut self, column: usize, by: usize) {
        let mut col = vec![false; self.pixels.len()];

        for (old_idx, row) in self.pixels.iter().enumerate() {
            let new_idx = (old_idx + by) % self.pixels.len();
            col[new_idx] = row[column];
        }

        for (idx, pixel) in col.iter().enumerate() {
            self.pixels[idx][column] = *pixel;
        }
    }

    pub fn count_active_pixels(&self) -> usize {
        self.pixels.iter().flatten().filter(|&pixel| *pixel).count()
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = false;
        for row in &self.pixels {
            if w {
                writeln!(f)?
            }
            w = true;
            for pixel in row {
                write!(f, "{}", if *pixel { '#' } else { ' ' })?
            }
        }
        Ok(())
    }
}
