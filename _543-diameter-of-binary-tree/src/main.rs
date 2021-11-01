use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    })));
    let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: None,
    })));
    let leaf3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: leaf1,
        right: leaf2,
    })));
    let leaf4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: leaf3,
        right: leaf4,
    })));
    assert_eq!(diameter_of_binary_tree(root), 3);
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let max = diameter(&root);
    max
}
fn diameter(tree: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max: i32 = 0;
    max_depth(tree, &mut max);
    max
}

fn max_depth(tree: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    if let Some(node) = tree {
        let node = node.borrow();
        let left = max_depth(&node.left, max);
        let right = max_depth(&node.right, max);
        *max = (*max).max(left + right);
        (left + 1).max(right + 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let leaf3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: leaf1,
            right: leaf2,
        })));
        let leaf4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: leaf3,
            right: leaf4,
        })));
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_2() {
        let leaf = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: leaf,
            right: None,
        })));
        assert_eq!(diameter_of_binary_tree(root), 1);
    }
}
