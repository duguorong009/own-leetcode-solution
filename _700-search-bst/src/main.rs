fn main() {
    println!("Hello, world!");
}

use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[macro_export]
macro_rules! tree {
    ($e:expr) => {
        TreeLink::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
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

trait Search {
    fn find(&self, val: i32) -> TreeLink;
}

impl Search for TreeLink {
    fn find(&self, val: i32) -> TreeLink {
        if let Some(node) = self {
            let temp = node.clone();
            let node = node.borrow();
            if val == node.val {
                Some(temp)
            } else {
                if val < node.val {
                    Self::find(&node.left, val)
                } else {
                    Self::find(&node.right, val)
                }
            }
        } else {
            None
        }
    }
}

struct Solution;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>, 
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.find(val)
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(3));
        let res = tree!(2, tree!(1), tree!(3));
        assert_eq!(Solution::search_bst(root, 2), res);
    }
}