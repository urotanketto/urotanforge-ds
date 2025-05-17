# urotanforge-ds

Part of the `urotanforge-` series:
a practical computer science project focused on data structure implementation in Rust.

---

## Implemented

- [x] Stack (LIFO)
- [x] Queue (FIFO)

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

## Queue

A **Queue** is a linear data strucuture that follow the **First-In First-Out** (FIFO) principle.
This implementation uses `VecDeque<T>` to efficiently support operations at both ends.

### Available operations

- `enqueue(item)`: Add an item to the back
- `dequeue()`: Remove the item from the front
- `peek()`: View the front item without removing
- `is_empty()`: Check if the queue is empty
- `size()`: Get the number of items in the queue
- `clear()`: Remove all elements from the queue

### Example

```rust
let mut queue = Queue::new();
queue.enqueue("first");
queue.enqueue("second");

assert_eq!(queue.dequeue(), Some("first"));
```
