use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

    assert_eq!(preorder_traverse_using_iteration(root), vec![1, 2, 3]);
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

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
        res.push(node_val);
        traverse(left_node, res);
        traverse(right_node, res);
    } else {
        return;
    }
}

fn preorder_traverse_using_iteration(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        if let Some(node) = queue.pop_front().unwrap() {
            res.push(node.borrow().val);
            queue.push_back(node.borrow_mut().left.take());
            queue.push_back(node.borrow_mut().right.take());
        }
    }
    res
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

        assert_eq!(preorder_traversal(root), vec![1, 2, 3]);
    }
}
