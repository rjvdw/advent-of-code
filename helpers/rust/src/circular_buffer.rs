//! A circular buffer that is implemented using a linked list.

use std::{fmt, mem};
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

/// A circular buffer that is implemented using a linked list.
#[derive(Debug, Clone)]
pub struct CircularBuffer<T> {
    forward: Vec<usize>,
    backward: Vec<usize>,
    elements: Vec<T>,
    current: usize,
    free: HashSet<usize>,
    length: usize,
}

impl<T> CircularBuffer<T> {
    /// Creates a new circular buffer from the provided elements. This list must not be empty.
    pub fn new(from: Vec<T>) -> CircularBuffer<T> {
        if from.is_empty() {
            panic!("Cannot create an empty circular buffer.");
        }

        let length = from.len();

        let mut cb = CircularBuffer {
            forward: vec![0; length],
            backward: vec![length - 1; length],
            elements: from,
            current: 0,
            free: HashSet::new(),
            length: length,
        };

        for idx in 0..length - 1 {
            cb.forward[idx] = idx + 1;
            cb.backward[idx + 1] = idx;
        }

        cb
    }

    /// Returns the length of this buffer.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns a reference to the current value.
    pub fn get_current(&self) -> &T {
        &self.elements[self.current]
    }

    /// Returns a mutable reference to the current value.
    pub fn get_current_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }

    /// Returns a reference to the specified value.
    pub fn get(&self, idx: usize) -> Option<&T> {
        Some(&self[idx])
    }

    /// Returns a mutable reference to the specified value.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        Some(&mut self[idx])
    }

    /// Rotates the buffer to the right by the specified number of steps.
    pub fn rotate_right(&mut self, by: usize) {
        for _ in 0..by {
            self.current = self.backward[self.current];
        }
    }

    /// Rotates the buffer to the left by the specified number of steps.
    pub fn rotate_left(&mut self, by: usize) {
        for _ in 0..by {
            self.current = self.forward[self.current];
        }
    }

    /// Rotates the buffer so that it starts with the first element that matches the provided
    /// predicate. Returns false if such an element could not be found.
    pub fn rotate_to<P>(&mut self, predicate: P) -> bool
    where
        P: Fn(&T) -> bool,
    {
        if let Some(idx) = self.elements.iter().position(predicate) {
            self.current = idx;
            true
        } else {
            false
        }
    }

    /// Inserts a value after the current position.
    pub fn insert(&mut self, value: T) {
        self.length += 1;
        let next = self.forward[self.current];
        let new_idx = if let Some(free) = self.free.iter().copied().next() {
            self.free.remove(&free);
            self.forward[free] = next;
            self.backward[free] = self.current;
            self.elements[free] = value;
            free
        } else {
            let idx = self.elements.len();
            self.forward.push(next);
            self.backward.push(self.current);
            self.elements.push(value);
            idx
        };

        self.forward[self.current] = new_idx;
        self.backward[next] = new_idx;
    }

    /// Creates a vec representing this circular buffer.
    pub fn to_vec(&self) -> Vec<&T> {
        let mut v = Vec::with_capacity(self.len());
        v.push(&self.elements[self.current]);
        let mut current = self.forward[self.current];

        // As a safe guard, we keep track of the number of iterations. If for some reason we are
        // doing more iterations than the expected length, something went wrong, and we could be
        // entering an infinite loop. In that case, we will panic.
        let mut safety = self.len() + 1;

        while current != self.current {
            safety -= 1;
            if safety == 0 {
                panic!("Something went wrong which caused us the potentially enter an infinite loop.");
            }
            v.push(&self.elements[current]);
            current = self.forward[current];
        }

        v
    }
}

impl<T: Default + Sized> CircularBuffer<T> {
    /// Removes the value at the current position.
    pub fn remove_current(&mut self, n: usize) -> Vec<T> {
        self.remove_offset(0, n)
    }

    /// Removes n values at the specified offset.
    pub fn remove_offset(&mut self, offset: usize, mut n: usize) -> Vec<T> {
        if self.length < n + 1 {
            panic!("Removing {} elements would cause this circular buffer to become empty.", n);
        }

        self.length -= n;

        let mut current = self.current;
        for _ in 0..offset {
            current = self.forward[self.current];
        }

        let mut removed = Vec::with_capacity(n);

        while n > 0 {
            n -= 1;

            // Steal the value from the elements, replacing it with a default value. This is under the
            // assumption that default values are probably smaller, thereby freeing up some memory.
            let mut value = T::default();
            mem::swap(&mut value, &mut self.elements[current]);

            self.free.insert(current);

            let prev = self.backward[current];
            let next = self.forward[current];
            self.backward[next] = prev;
            self.forward[prev] = next;
            if self.current == current {
                self.current = prev;
            }
            current = prev;

            removed.push(value);
        }

        removed
    }
}

impl<T> Index<usize> for CircularBuffer<T> {
    type Output = T;

    /// Returns a reference to the nth element (complexity O(n)).
    fn index(&self, n: usize) -> &Self::Output {
        let mut current = 0;
        for _ in 0..n % self.len() {
            current = self.forward[current];
        }
        &self.elements[current]
    }
}

impl<T> IndexMut<usize> for CircularBuffer<T> {
    /// Returns a mutable reference to the nth element (complexity O(n)).
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        let mut current = 0;
        for _ in 0..n % self.len() {
            current = self.forward[current];
        }
        &mut self.elements[current]
    }
}

impl<T: fmt::Display> fmt::Display for CircularBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (idx, el) in self.to_vec().iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            el.fmt(f)?;
        }
        write!(f, "]")
    }
}
