use urotanforge_ds::queue::Queue;

#[test]
fn test_queue_behavior() {
    let mut queue = Queue::new();
    assert!(queue.is_empty());

    queue.enqueue("first");
    queue.enqueue("second");

    assert_eq!(queue.peek(), Some(&"first"));
    assert_eq!(queue.dequeue(), Some("first"));
    assert_eq!(queue.dequeue(), Some("second"));
    assert!(queue.is_empty());
}
