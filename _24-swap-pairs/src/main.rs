use std::borrow::{BorrowMut, Borrow};

fn main() {
    println!("Hello, world!");
}

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
macro_rules! ListLink {
    ($e: expr) => {
        Some(Box::new(ListNode { val: $e, next: None }))
    };
    ($e: expr, $next: expr) => {
        Some(Box::new(ListNode { val: $e, next: $next }))
    };
}

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut first) = head {
        if let Some(mut second) = first.next.take() {
            let rest = second.next.take();
            first.next = swap_pairs(rest);
            second.next = Some(first);
            Some(second)
        } else {
            Some(first)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = ListLink!(1, ListLink!(2, ListLink!(3, ListLink!(4))));
        let expected = ListLink!(2, ListLink!(1, ListLink!(4, ListLink!(3))));
        assert_eq!(swap_pairs(head), expected);
    }
}