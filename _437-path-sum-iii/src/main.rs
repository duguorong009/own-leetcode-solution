use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
        val: -2,
        left: None,
        right: None,
    })));
    let node1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: leaf1,
        right: leaf2,
    })));
    let leaf3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    let node2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: leaf3,
    })));
    let node3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: node1,
        right: node2,
    })));

    let leaf4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 11,
        left: None,
        right: None,
    })));
    let node4 = Some(Rc::new(RefCell::new(TreeNode {
        val: -3,
        left: None,
        right: leaf4,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: node3,
        right: node4,
    })));
    let target_sum = 8;
    assert_eq!(path_sum(root, target_sum), 3);
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

fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    let prefix_map: &mut HashMap<i32, i32> = &mut HashMap::new();
    prefix_map.insert(0, 1);
    calc_path_sum(&root, 0, target_sum, prefix_map)
}

fn calc_path_sum(
    node: &Option<Rc<RefCell<TreeNode>>>,
    prefix: i32,
    sum: i32,
    prefix_map: &mut HashMap<i32, i32>,
) -> i32 {
    if let Some(node) = node {
        let node = node.borrow();
        let left = &node.left;
        let right = &node.right;
        let val = node.val;
        let mut res = 0;
        let prefix = prefix + val;
        let count = *prefix_map.get(&(prefix - sum)).unwrap_or(&0);
        res += count;
        *prefix_map.entry(prefix).or_default() += 1;
        res += calc_path_sum(left, prefix, sum, prefix_map);
        res += calc_path_sum(right, prefix, sum, prefix_map);
        *prefix_map.entry(prefix).or_default() -= 1;
        res
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_1() {
        let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
            val: -2,
            left: None,
            right: None,
        })));
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: leaf1,
            right: leaf2,
        })));
        let leaf3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: leaf3,
        })));
        let node3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: node1,
            right: node2,
        })));

        let leaf4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 11,
            left: None,
            right: None,
        })));
        let node4 = Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: None,
            right: leaf4,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: node3,
            right: node4,
        })));
        let target_sum = 8;
        assert_eq!(path_sum(root, target_sum), 3);
    }
}
