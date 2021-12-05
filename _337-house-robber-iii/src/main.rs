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

fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (l, r) = postorder(&root);
    l.max(r)
}

fn postorder(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(node) = node {
        let node = node.borrow();
        let val = node.val;
        let left = postorder(&node.left);
        let right = postorder(&node.right);
        (
            val + left.1 + right.1,
            left.0.max(left.1) + right.0.max(right.1),
        )
    } else {
        (0, 0)
    }
}

// This version is just for maximizing the profit when considering tree levels.
fn my_own_rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root = root;
    let tree_height = height(&root);
    let mut profits: Vec<i32> = vec![0];
    for i in 1..=tree_height {
        let mut sum = 0;
        sum_current_level(&root, i, &mut sum);
        profits.push(sum);
    }

    let mut dp: Vec<i32> = vec![profits[0], profits[1]];

    for i in 2..=tree_height {
        let max_val = dp[i - 1].max(dp[i - 2] + profits[i]);
        dp.push(max_val);
    }

    dp[tree_height]
}

/* Compute the "height" of a tree -- the number of
nodes along the longest path from the root node
down to the farthest leaf node.*/
fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    if let Some(node) = node {
        let node = node.borrow();
        let l_height = height(&node.left);
        let r_height = height(&node.right);
        if l_height <= r_height {
            r_height + 1
        } else {
            l_height + 1
        }
    } else {
        0
    }
}

/* Sum nodes at the current level */
fn sum_current_level(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, sum: &mut i32) {
    if let Some(node) = node {
        let node = node.borrow();
        if level == 1 {
            *sum += node.val;
        } else if level > 1 {
            sum_current_level(&node.left, level - 1, sum);
            sum_current_level(&node.right, level - 1, sum);
        }
    } else {
        return;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let leaf1 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let node1 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(leaf1),
        }));

        let leaf2 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(leaf2),
        }));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(node1),
            right: Some(node2),
        })));
        assert_eq!(rob(root), 7);
    }

    #[test]
    fn test_2() {
        let leaf1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));
        let leaf2 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let node1 = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(leaf1),
            right: Some(leaf2),
        }));

        let leaf3 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(leaf3),
        }));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(node1),
            right: Some(node2),
        })));
        assert_eq!(rob(root), 9);
    }
}
