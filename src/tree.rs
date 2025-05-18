/// A basic binary tree node storing a value and two children.
///
/// This version is not a Binary Search Tree (BST).
/// It stores nodes in arbitrary left/right positions.
#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    /// Create a new leaf node.
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Insert a node to the left.
    pub fn insert_left(&mut self, value: T) {
        self.left = Some(Box::new(TreeNode::new(value)));
    }

    /// Insert a node to the right.
    pub fn insert_right(&mut self, value: T) {
        self.right = Some(Box::new(TreeNode::new(value)));
    }

    /// In-order traversal: left -> root -> right
    pub fn in_order_traversal(&self, visit: &mut impl FnMut(&T)) {
        if let Some(ref left) = self.left {
            left.in_order_traversal(visit);
        }
        visit(&self.value);
        if let Some(ref right) = self.right {
            right.in_order_traversal(visit);
        }
    }
}
