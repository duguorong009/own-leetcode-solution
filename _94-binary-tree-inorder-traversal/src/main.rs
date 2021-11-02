use std::cell::{Ref, RefCell};
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

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    traverse(&root, &mut res);
    res
}

fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node) = node {
        let node = node.borrow();
        let left_node = &node.left;
        let right_node = &node.right;
        let node_val = node.val;
        traverse(left_node, res);
        res.push(node_val);
        traverse(right_node, res);
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let node_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let node_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node_3,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node_2,
        })));

        assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let root = None;
        assert_eq!(inorder_traversal(root).len(), 0);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        assert_eq!(inorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_4() {}
}
