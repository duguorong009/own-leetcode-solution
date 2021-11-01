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

fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut cnt = 0;
    count_nodes_in_node(&root, &mut cnt);
    cnt
}

fn count_nodes_in_node(node: &Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) {
    if let Some(node) = node {
        *cnt += 1;
        let node = node.borrow();
        let left = &node.left;
        let right = &node.right;
        count_nodes_in_node(left, cnt);
        count_nodes_in_node(right, cnt);
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let node_4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));

        let node_5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));

        let node_6 = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })));

        let node_2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node_4,
            right: node_5,
        })));
        let node_3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node_6,
            right: None,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node_2,
            right: node_3,
        })));

        assert_eq!(count_nodes(root), 6);
    }

    #[test]
    fn test_2() {
        let root = None;
        assert_eq!(count_nodes(root), 0);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(count_nodes(root), 1);
    }
}
