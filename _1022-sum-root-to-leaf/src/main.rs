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

fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
        let partial_sum = partial_sum * 2 + node_val;
        if left_child == &None && right_child == &None {
            *res += partial_sum;
        }
        dfs(left_child, res, partial_sum);
        dfs(right_child, res, partial_sum);
    } else {
        return;
    }
}
