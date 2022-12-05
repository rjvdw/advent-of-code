use std::collections::VecDeque;

pub trait Crane {
    fn push(&mut self, c: char);
    fn pop(&mut self) -> Option<char>;
}

#[derive(Debug, Default)]
pub struct CrateMover9000 {
    stack: VecDeque<char>,
}

impl Crane for CrateMover9000 {
    fn push(&mut self, c: char) {
        self.stack.push_front(c)
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop_back()
    }
}

#[derive(Debug, Default)]
pub struct CrateMover9001 {
    stack: Vec<char>,
}

impl Crane for CrateMover9001 {
    fn push(&mut self, c: char) {
        self.stack.push(c)
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }
}
