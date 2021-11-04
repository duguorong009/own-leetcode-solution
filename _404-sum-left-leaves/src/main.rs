use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
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

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    dfs(&root, "root", &mut res);
    res
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, node_tag: &str, res: &mut i32) {
    if let Some(node) = node {
        let node = node.borrow();
        if node.left == None && node.right == None && node_tag == "l" {
            *res += node.val;
        }
        dfs(&node.left, "l", res);
        dfs(&node.right, "r", res);
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let leaf_9 = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })));
        let leaf_15 = Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: None,
            right: None,
        })));
        let leaf_7 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node_20 = Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: leaf_15,
            right: leaf_7,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: leaf_9,
            right: node_20,
        })));
        assert_eq!(sum_of_left_leaves(root), 24);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(sum_of_left_leaves(root), 0);
    }
}
