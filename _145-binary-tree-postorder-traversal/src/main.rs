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
    assert_eq!(postorder_traversal(root), vec![3, 2, 1]);
}

fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    traverse(&root, &mut res);
    res
}

fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node) = node {
        let node = node.borrow();
        traverse(&node.left, res);
        traverse(&node.right, res);
        res.push(node.val);
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
        assert_eq!(postorder_traversal(root), vec![3, 2, 1]);
    }
}
