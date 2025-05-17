/// A simple stack implementation using `Vec<T>`
///
/// Stack is a Last-In First-Out (LIFO) data structure.
/// Typical operations are:
/// - `push`: insert an element
/// - `pop`: remove the most recently inserted element
/// - `peek`: look at the top element without removing it
#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new, empty stack.
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Push an Item onto the stack.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pop the top item off the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Peek at the top item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Check whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items in the stack.
    pub fn size(&self) -> usize {
        self.items.len()
    }
}
