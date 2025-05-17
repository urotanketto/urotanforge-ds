use urotanforge_ds::stack::Stack;

#[test]
fn test_stack_behavior() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());

    stack.push(42);
    stack.push(99);

    assert_eq!(stack.peek(), Some(&99));
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.pop(), Some(99));
    assert_eq!(stack.pop(), Some(42));
    assert!(stack.is_empty());
}

#[test]
fn test_stack_clear() {
    let mut stack = Stack::new();
    stack.push("a");
    stack.push("b");
    assert_eq!(stack.size(), 2);

    stack.clear();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), None);
}
