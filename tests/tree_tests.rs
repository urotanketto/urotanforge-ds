use urotanforge_ds::tree::TreeNode;

#[test]
fn test_in_order_traversal() {
    let mut root = TreeNode::new(1);
    root.insert_left(2);
    root.insert_right(3);

    let mut visited = Vec::new();
    root.in_order_traversal(&mut |v| visited.push(*v));

    assert_eq!(visited, vec![2, 1, 3]);
}

#[test]
fn test_tree_contains() {
    let mut root = TreeNode::new(10);
    root.insert_left(5);
    root.insert_right(15);

    assert!(root.contains(&10));
    assert!(root.contains(&5));
    assert!(root.contains(&15));
    assert!(!root.contains(&99));
}
