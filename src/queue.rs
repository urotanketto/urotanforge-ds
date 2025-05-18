use std::collections::VecDeque;

/// A simple queue implementation using `VecDeque<T>`.
///
/// Queue is a First-In First-Out (FIFO) data structure.
/// Typical operations include:
/// - `enqueue`: add an item to the back
/// - `dequeue`: remove the item from the front
/// - `peek`: view the front item without removing it
#[derive(Debug, Clone)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    /// Create a new empty queue.
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    /// Add an item to the back of the queue.
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    /// Remove and return the front item.
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// View the front item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items in the queue.
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Remove all elements from the queue.
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
