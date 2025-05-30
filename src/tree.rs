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

impl<T: Ord> TreeNode<T> {
    /// Create a new leaf node.
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Insert a value into the binary search tree.
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
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

    /// Check whether the tree contains the given value.
    pub fn contains(&self, target: &T) -> bool {
        if &self.value == target {
            true
        } else if target < &self.value {
            self.left
                .as_ref()
                .map_or(false, |node| node.contains(target))
        } else {
            self.right
                .as_ref()
                .map_or(false, |node| node.contains(target))
        }
    }
}
