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
