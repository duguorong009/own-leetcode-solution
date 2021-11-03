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

fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    dfs(&root, &mut res, 0);
    res
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32, partial_sum: i32) {
    if let Some(node) = node {
        let node = node.as_ref().borrow();
        let left_child = &node.left;
        let right_child = &node.right;
        let node_val = node.val;
        let partial_sum = partial_sum * 10 + node_val;
        if left_child == &None && right_child == &None {
            *res += partial_sum;
        }
        dfs(left_child, res, partial_sum);
        dfs(right_child, res, partial_sum);
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{sum_numbers, TreeNode};

    #[test]
    fn test_1() {
        let node_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let node_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node_2,
            right: node_3,
        })));
        assert_eq!(sum_numbers(root), 25);
    }

    #[test]
    fn test_2() {
        let node_5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node_1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node_9 = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: node_5,
            right: node_1,
        })));
        let node_0 = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node_9,
            right: node_0,
        })));
        assert_eq!(sum_numbers(root), 1026);
    }

    #[test]
    fn test_3() {
        let node_1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node_9 = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: node_1,
        })));
        let node_0 = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node_9,
            right: node_0,
        })));
        assert_eq!(sum_numbers(root), 531);
    }
}
