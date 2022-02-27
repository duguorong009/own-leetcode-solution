use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

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

trait Preorder {
    fn preorder(
        &self, 
        row: usize, 
        pos: u32, 
        min: &mut HashMap<usize, u32>, 
        max: &mut HashMap<usize, u32>, 
        diff: &mut u32,
    );
}

impl Preorder for Option<Rc<RefCell<TreeNode>>> {
    fn preorder(
        &self,
        row: usize,
        pos: u32,
        min: &mut HashMap<usize, u32>,
        max: &mut HashMap<usize, u32>,
        diff: &mut u32,
    ) {
        if let Some(node) = self {
            min.entry(row).or_insert(pos);
            max.entry(row).or_insert(pos);
            *min.get_mut(&row).unwrap() = min[&row].min(pos);
            *max.get_mut(&row).unwrap() = max[&row].max(pos);
            *diff = (*diff).max(max[&row] - min[&row] + 1);
            let node = node.borrow();
            node.left.preorder(row + 1, pos << 1, min, max, diff);
            node.right.preorder(row + 1, (pos << 1) + 1, min, max, diff);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut min: HashMap<usize, u32> = HashMap::new();
    let mut max: HashMap<usize, u32> = HashMap::new();
    let mut res = 0;
    root.preorder(0, 0, &mut min, &mut max, &mut res);
    res as i32
}

#[cfg(test)]
mod tests {}