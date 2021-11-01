use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
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

fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut table: HashMap<i32, (i32, i32)> = HashMap::new(); // HashMap: node_value => (depth, parent_value)
    fill_table(&mut table, &root, 0, i32::MAX);
    let x_res = table.get(&x).unwrap();
    let y_res = table.get(&y).unwrap();
    println!("{:?}", table);
    if x_res.0 == y_res.0 && x_res.1 != y_res.1 {
        true
    } else {
        false
    }
}

fn fill_table(
    table: &mut HashMap<i32, (i32, i32)>,
    node: &Option<Rc<RefCell<TreeNode>>>,
    depth: i32,
    parent_value: i32,
) {
    if let Some(node) = node {
        let node = node.try_borrow().unwrap();
        let left_node = &node.left;
        let right_node = &node.right;
        if depth == 0 {
            table.insert(node.val, (depth, node.val));
        } else {
            table.insert(node.val, (depth, parent_value));
        }
        fill_table(table, left_node, depth + 1, node.val);
        fill_table(table, right_node, depth + 1, node.val);
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let left2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));

        let left1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: left2,
            right: None,
        })));

        let right1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: left1,
            right: right1,
        })));

        let x = 4;
        let y = 3;
        assert_eq!(is_cousins(root, x, y), false);
    }

    #[test]
    fn test_2() {
        let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));

        let left1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: leaf1,
        })));

        let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));

        let right1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: leaf2,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: left1,
            right: right1,
        })));

        let x = 5;
        let y = 4;
        assert_eq!(is_cousins(root, x, y), true);
    }

    #[test]
    fn test_3() {
        let right2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));

        let left1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: right2,
        })));

        let right1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: left1,
            right: right1,
        })));

        let x = 2;
        let y = 3;
        assert_eq!(is_cousins(root, x, y), false);
    }
}
