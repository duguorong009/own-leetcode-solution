use std::cell::RefCell;
use std::cmp::Ordering::*;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[macro_export]
macro_rules! tree {
    ($e: expr) => {
        TreeLink::leaf($e)
    };
    ($e: expr, $l: expr, $r: expr) => {
        TreeLink::branch($e, $l, $r)
    };
}

pub trait TreeMaker {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl TreeMaker for TreeLink {}

fn main() {
    println!("Hello, world!");
}

fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    delete(root, key)
}

fn delete(node: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node {
        let mut node = node.borrow_mut();
        let left = node.left.take();
        let right = node.right.take();
        let val = node.val;
        match key.cmp(&val) {
            Equal => match (left, right) {
                (None, None) => None,
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (left, right) => {
                    let smallet = smallest(&right);
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: smallet,
                        left,
                        right: delete(right, smallet),
                    })))
                }
            },
            Less => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: delete(left, key),
                right,
            }))),
            Greater => Some(Rc::new(RefCell::new(TreeNode {
                val,
                left,
                right: delete(right, key),
            }))),
        }
    } else {
        None
    }
}

fn smallest(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = node {
        let node = node.borrow();
        let val = node.val;
        if node.left.is_some() {
            smallest(&node.left)
        } else {
            val
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = tree!(5, tree!(3, tree!(2), tree!(4)), tree!(6, None, tree!(7)));
        let key = 3;
        let res = tree!(5, tree!(4, tree!(2), None), tree!(6, None, tree!(7)));
        assert_eq!(delete_node(root, key), res);
    }
}
