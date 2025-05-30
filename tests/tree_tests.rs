use urotanforge_ds::tree::TreeNode;

#[test]
fn test_bst_insert_and_traversal() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(7);
    root.insert(3);

    let mut visited = Vec::new();
    root.in_order_traversal(&mut |v| visited.push(*v));

    assert_eq!(visited, vec![3, 5, 7, 10, 15]);
}

#[test]
fn test_bst_contains() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);

    assert!(root.contains(&10));
    assert!(root.contains(&5));
    assert!(root.contains(&15));
    assert!(!root.contains(&99));
}
