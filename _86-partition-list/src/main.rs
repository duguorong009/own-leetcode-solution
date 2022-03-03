use std::borrow::Borrow;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[macro_export]
macro_rules!  ListLink {
    ($v: expr) => {
        Some(Box::new(ListNode { val: $v, next: None }))
    };
    ($v: expr, $next: expr) => {
        Some(Box::new(ListNode { val: $v, next: $next }))
    }
}

fn main() {
    println!("Hello, world!");
}

fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    // Convert the linked list to digit array.
    let mut arr: Vec<i32> = vec![];
    let mut temp: &Option<Box<ListNode>> = &head;
    while temp.is_some() {
        let node = temp.borrow().as_ref().unwrap();
        arr.push(node.val);
        temp = &node.next;
    }

    // Partition the array.
    partition_arr(&mut arr, x);
    
    // Re-convert the array to linked list
    let mut res: Option<Box<ListNode>> = None;
    for &val in arr.iter().rev() {
        res = Some(Box::new(ListNode { val, next: res }));
    }

    res
}

fn partition_arr(arr: &mut Vec<i32>, x: i32) {
    let n = arr.len();
    let mut before: Vec<i32> = vec![];
    let mut after: Vec<i32> = vec![];
    for i in 0..n {
        if arr[i] < x {
            before.push(arr[i]);
        } else {
            after.push(arr[i]);
        }
    }
    
    let before_len = before.len();
    for i in 0..before_len {
        arr[i] = before[i];
    }

    for i in 0..after.len() {
        arr[i + before_len] = after[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = ListLink!(1, ListLink!(4, ListLink!(3, ListLink!(2, ListLink!(5, ListLink!(2))))));
        let expected = ListLink!(1, ListLink!(2, ListLink!(2, ListLink!(4, ListLink!(3, ListLink!(5))))));
        assert_eq!(partition(head, 3), expected);
    }
}