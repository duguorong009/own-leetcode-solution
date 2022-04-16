use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

macro_rules! tree {
    ($v:expr, $left:expr, $right: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: $left,
            right: $right,
        })))
    };
    ($v: expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            val: $v,
            left: None,
            right: None,
        })))
    };
}

struct Solution;

impl Solution {
    pub fn convert_bst(root: TreeLink) -> TreeLink {
        let mut inorder_traverse: Vec<i32> = vec![];
        let mut hm: HashMap<i32, i32> = HashMap::new();

        Solution::inorder_traversal(&root, &mut inorder_traverse);

        Solution::build_hm(&inorder_traverse, &mut hm);

        Solution::replace_values(&root, &hm);

        root
    }

    pub fn inorder_traversal(node: &TreeLink, arr: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Solution::inorder_traversal(&node.left, arr);
            arr.push(node.val);
            Solution::inorder_traversal(&node.right, arr);
        } else {
            return;
        }
    }
    pub fn replace_values(node: &TreeLink, hm: &HashMap<i32, i32>) {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            let k = node.val;
            Solution::replace_values(&node.left, hm);
            node.val = *hm.get(&k).unwrap();
            Solution::replace_values(&node.right, hm);
        } else {
            return;
        }
    }

    pub fn build_hm(arr: &Vec<i32>, hm: &mut HashMap<i32, i32>) {
        let n = arr.len();
        for (i, &v) in arr.iter().enumerate() {
            let sum: i32 = arr[i..n].iter().sum();
            hm.insert(v, sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = tree!(
            4,
            tree!(1, tree!(0), tree!(2, None, tree!(3))),
            tree!(6, tree!(5), tree!(7, None, tree!(8)))
        );
        let expected = tree!(
            30,
            tree!(36, tree!(36), tree!(35, None, tree!(33))),
            tree!(21, tree!(26), tree!(15, None, tree!(8)))
        );
        assert_eq!(Solution::convert_bst(root), expected);
    }
}
