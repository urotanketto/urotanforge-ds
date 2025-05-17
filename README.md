# urotanforge-ds

Part of the `urotanforge-` series:
a practical computer science project focused on data structure implementation in Rust.

---

## Implemented

- [x] Stack (LIFO)

---

## Stack

A **Stack** is a linear data structure that follows the **Last-In First-Out** (LIFO) principle.
This implementation uses a generic `Vec<T>` internally.

### Available operations

- `push(item)`: Add an item to the top
- `pop()`: Remove the top item
- `peek()`: View the top item without removing
- `is_empty()`: Check if the stack is empty
- `size()`: Get the number of items in the stack
- `clear()`: Remove all elements from the stack

### Example

```rust
let mut stack = Stack::new();
stack.push("apple");
stack.push("banana");

assert_eq!(stack.pop(), Some("banana"));
```
