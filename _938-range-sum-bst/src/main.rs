use std::cell::RefCell;
use std::rc::Rc;
type TreeLink = Option<Rc<RefCell<TreeNode>>>;
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

fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut sum = 0;
    traverse_tree_calc_sum(&root, &mut sum, low, high);
    sum
}

fn traverse_tree_calc_sum(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sum: &mut i32,
    low: i32,
    high: i32,
) {
    if let Some(node) = node {
        let node = node.borrow();
        if low <= node.val && node.val <= high {
            *sum += node.val;
        }
        traverse_tree_calc_sum(&node.left, sum, low, high);
        traverse_tree_calc_sum(&node.right, sum, low, high);
    } else {
        return;
    }
}
