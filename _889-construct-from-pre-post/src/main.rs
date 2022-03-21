use std::rc::Rc;
use std::cell::RefCell;

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


fn construct_from_pre_post(
    preorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    build(&mut 0, &mut 0, &preorder, &postorder)
}

fn build(i: &mut usize, j: &mut usize, pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let val = pre[*i];
    *i += 1;
    let mut left = None;
    let mut right = None;
    if val != post[*j] {
        left = build(i, j, pre, post);
    }
    if val != post[*j] {
        right = build(i, j, pre, post);
    }
    *j += 1;
    tree!(val, left, right)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let pre = vec![1, 2, 4, 5, 3, 6, 7];
        let post = vec![4, 5, 2, 6, 7, 3, 1];
        let res = tree!(
            1,
            tree!(2, tree!(4), tree!(5)),
            tree!(3, tree!(6), tree!(7))
        );
        assert_eq!(construct_from_pre_post(pre, post), res);
    }

    #[test]
    fn test_2() {
        let preorder = vec![1];
        let postorder = vec![1];
        let expected_tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None, 
            right: None,
        })));
        assert_eq!(construct_from_pre_post(preorder, postorder), expected_tree);
    }
}