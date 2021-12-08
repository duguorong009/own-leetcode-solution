use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {
    println!("Hello, world!");
}

fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum: i32 = 0;
    traverse_tree(&root, &mut sum);
    sum
}

fn traverse_tree(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
    if let Some(node) = node {
        let node = node.borrow();
        let mut left_sum = 0;
        tree_sum(&node.left, &mut left_sum);
        let mut right_sum = 0;
        tree_sum(&node.right, &mut right_sum);
        *sum += i32::abs(left_sum - right_sum);

        traverse_tree(&node.left, sum);
        traverse_tree(&node.right, sum);
    } else {
        return;
    }
}

fn tree_sum(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
    if let Some(node) = node {
        let node = node.borrow();
        *sum += node.val;
        tree_sum(&node.left, sum);
        tree_sum(&node.right, sum);
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node1,
            right: node2,
        })));
        assert_eq!(find_tilt(root), 1);
    }
}
