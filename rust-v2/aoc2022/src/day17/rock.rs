use crate::point::Point;

#[derive(Debug, Copy, Clone)]
pub enum Rock {
    HorizontalLine,
    Plus,
    Corner,
    VerticalLine,
    Square,
}

impl Rock {
    /// Returns the nth rock that will fall.
    pub fn nth(n: usize) -> Rock {
        match n % 5 {
            0 => Rock::HorizontalLine,
            1 => Rock::Plus,
            2 => Rock::Corner,
            3 => Rock::VerticalLine,
            _ => Rock::Square,
        }
    }

    /// The width of the rock.
    pub fn width(&self) -> usize {
        match self {
            Rock::HorizontalLine => 4,
            Rock::Plus => 3,
            Rock::Corner => 3,
            Rock::VerticalLine => 1,
            Rock::Square => 2,
        }
    }

    /// The height of the rock.
    pub fn height(&self) -> usize {
        match self {
            Rock::HorizontalLine => 1,
            Rock::Plus => 3,
            Rock::Corner => 3,
            Rock::VerticalLine => 4,
            Rock::Square => 2,
        }
    }

    /// Returns a list of points that are contained by this rock.
    ///
    /// * `position` - The current x,y-coordinate of the rock, where x indicates the distance to the wall on the left and y indicates the distance to the floor.
    pub fn points(&self, position: Point) -> Vec<Point> {
        match self {
            Rock::HorizontalLine => vec![
                position,
                Point(position.0 + 1, position.1),
                Point(position.0 + 2, position.1),
                Point(position.0 + 3, position.1),
            ],
            Rock::Plus => vec![
                Point(position.0 + 1, position.1),
                Point(position.0, position.1 + 1),
                Point(position.0 + 1, position.1 + 1),
                Point(position.0 + 2, position.1 + 1),
                Point(position.0 + 1, position.1 + 2),
            ],
            Rock::Corner => vec![
                position,
                Point(position.0 + 1, position.1),
                Point(position.0 + 2, position.1),
                Point(position.0 + 2, position.1 + 1),
                Point(position.0 + 2, position.1 + 2),
            ],
            Rock::VerticalLine => vec![
                position,
                Point(position.0, position.1 + 1),
                Point(position.0, position.1 + 2),
                Point(position.0, position.1 + 3),
            ],
            Rock::Square => vec![
                position,
                Point(position.0, position.1 + 1),
                Point(position.0 + 1, position.1),
                Point(position.0 + 1, position.1 + 1),
            ],
        }
    }
}
