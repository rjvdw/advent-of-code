use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

const X: i8 = 3;
const Y: i8 = 2;
const Z: i8 = 1;

const NEG_X: i8 = -X;
const NEG_Y: i8 = -Y;
const NEG_Z: i8 = -Z;

/// Prints a list of all possible rotations of a 3D object.
fn main() {
    let mut rotations = HashSet::new();
    let mut point = Point::new(X, Y, Z);

    // Collect all possible rotations in a set, thereby eliminating duplicates.
    for _ in 0..4 {
        point = point.rotate_x();
        for _ in 0..4 {
            point = point.rotate_y();
            for _ in 0..4 {
                point = point.rotate_z();
                rotations.insert(point);
            }
        }
    }

    // Turn the set into a list, so the items can be sorted.
    let len = rotations.len();
    let mut rotations = rotations.iter().copied().collect::<Vec<Point>>();
    rotations.sort_unstable();

    // Print the match expression.
    println!("=== THE MATCH EXPRESSION IS ===");
    println!("match orientation {{");
    for (i, point) in rotations.iter().enumerate() {
        if i + 1 == len {
            println!("    _ => {},", point);
        } else {
            println!("    {} => {},", i, point);
        }
    }
    println!("}}");
    println!();

    // Print the test cases.
    println!("=== THE TEST CASES ARE ===");
    println!("#[test]");
    println!("fn test_rotations() {{");
    for (i, point) in rotations.iter().enumerate() {
        println!(
            "    assert_eq!(Point::new({}, {}, {}).rotate({}), Point::new({}, {}, {}));",
            X, Y, Z, i, point.x, point.y, point.z
        );
    }
    println!("}}");
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i8,
    y: i8,
    z: i8,
}

impl Point {
    fn new(x: i8, y: i8, z: i8) -> Point {
        Point { x, y, z }
    }

    /// Rotate a point by 90 degrees around the x-axis.
    fn rotate_x(&self) -> Point {
        Point::new(self.x, -self.z, self.y)
    }

    /// Rotate a point by 90 degrees around the y-axis
    fn rotate_y(&self) -> Point {
        Point::new(-self.z, self.y, self.x)
    }

    /// Rotate a point by 90 degrees around the z-axis.
    fn rotate_z(&self) -> Point {
        Point::new(-self.y, self.x, self.z)
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    /// Orders points using X > Y > Z > -Z > -Y > -X.
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .x
            .cmp(&self.x)
            .then(other.y.cmp(&self.y))
            .then(other.z.cmp(&self.z))
    }
}

impl fmt::Display for Point {
    /// Format a point. If the point is (X, Y, Z), just print `*self`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x == X && self.y == Y && self.z == Z {
            write!(f, "*self")
        } else {
            write!(
                f,
                "Point::new({}, {}, {})",
                as_xyz(self.x),
                as_xyz(self.y),
                as_xyz(self.z),
            )
        }
    }
}

/// Maps X to `self.x`, -X to `-self.x`, etc.
fn as_xyz(i: i8) -> &'static str {
    match i {
        X => "self.x",
        NEG_X => "-self.x",
        Y => "self.y",
        NEG_Y => "-self.y",
        Z => "self.z",
        NEG_Z => "-self.z",
        _ => panic!("No."),
    }
}
