pub type Label = String;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node(Label, Label);

impl Node {
    pub fn new(left: String, right: String) -> Node {
        Node(left, right)
    }

    pub fn travel(&self, direction: Direction) -> Label {
        match direction {
            Direction::Left => self.0.clone(),
            Direction::Right => self.1.clone(),
        }
    }
}
