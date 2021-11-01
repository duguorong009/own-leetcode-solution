use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let preorder = vec![8, 5, 1, 7, 10, 12];
    let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: None,
        right: None,
    })));
    let node1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: leaf1,
        right: leaf2,
    })));
    let leaf3 = Some(Rc::new(RefCell::new(TreeNode {
        val: 12,
        left: None,
        right: None,
    })));
    let node2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: None,
        right: leaf3,
    })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: node1,
        right: node2,
    })));
    assert_eq!(bst_from_preorder(preorder), root);
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

fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut inorder = preorder.clone();
    inorder.sort_unstable();
    bst_from_vec(&preorder, &inorder)
}

fn bst_from_vec(preorder: &Vec<i32>, inorder: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    // println!("Preorder:: {:?}", preorder);
    // println!("Inorder:: {:?}", inorder);
    let n = preorder.len();
    if n == 0 {
        None
    } else {
        if n == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: None,
                right: None,
            })))
        } else {
            let i = inorder.binary_search(&preorder[0]).unwrap();
            let left = bst_from_vec(&preorder[1..=i].to_vec(), &inorder[0..i].to_vec());
            let right = bst_from_vec(&preorder[i + 1..].to_vec(), &inorder[i + 1..].to_vec());
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[0],
                left: left,
                right: right,
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let preorder = vec![8, 5, 1, 7, 10, 12];
        let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let leaf2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        })));
        let node1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: leaf1,
            right: leaf2,
        })));
        let leaf3 = Some(Rc::new(RefCell::new(TreeNode {
            val: 12,
            left: None,
            right: None,
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: leaf3,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: node1,
            right: node2,
        })));
        assert_eq!(bst_from_preorder(preorder), root);
    }

    #[test]
    fn test_2() {
        let preorder = vec![1, 3];
        let leaf1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: leaf1,
        })));
        assert_eq!(bst_from_preorder(preorder), root);
    }
}
