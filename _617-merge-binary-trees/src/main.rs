use std::{cell::RefCell, rc::Rc};

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

fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Tree 1.
        let node1: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node2: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node1,
            right: None,
        })));
        let node3: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: node2,
            right: node3,
        })));

        // Tree 2.
        let node1: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let node2: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: node1,
        })));
        let node3: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node4: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: node3,
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node2,
            right: node4,
        })));

        // Expected Tree.
        let node7 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let node8 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let node9 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node5 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: node7,
            right: node8,
        })));
        let node6 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: node9,
        })));
        let expected_tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: node5,
            right: node6,
        })));
        assert_eq!(merge_trees(root1, root2), expected_tree);
    }

    #[test]
    fn test_2() {
        let root1: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        let node1: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let root2: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node1,
            right: None,
        })));

        let node1: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let expected_tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: node1,
            right: None,
        })));
        assert_eq!(merge_trees(root1, root2), expected_tree);
    }
}
