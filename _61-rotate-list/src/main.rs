
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
macro_rules! list {
    () => {
        None
    };
    ($e: expr) => {
        ListLink::link($e, None)
    };
    ($e: expr, $($tail:tt)*) => {
        ListLink::link($e, list!($($tail)*))
    };
}

pub type ListLink = Option<Box<ListNode>>;

pub trait ListMaker {
    fn link(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

impl ListMaker for ListLink {}

fn main() {
    println!("Hello, world!");
}

fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head= head;
    let mut p = head.as_ref();
    let mut n = 0;
    while let Some(node) = p {
        p = node.next.as_ref();
        n += 1;

    }
    if n < 2 {
        return head;
    }
    let k = k as usize % n;
    if k == 0 {
        return head;
    }
    let mut i = 0;
    let mut p = head.as_mut();
    let mut new_head: Option<Box<ListNode>> = None;
    while let Some(node) = p {
        if i + k == n - 1 {
            new_head = node.next.take();
            break;
        } else {
            p = node.next.as_mut();

        }
        i += 1;
    }
    let mut p = new_head.as_mut();
    while let Some(node) = p {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        p = node.next.as_mut();
    }
    new_head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = Some(Box::new(ListNode{ val: 1, next: Some(Box::new(ListNode {val: 2, next: Some(Box::new(ListNode{ val: 3, next: Some(Box::new(ListNode{ val: 4, next: Some(Box::new(ListNode { val: 5, next: None }))}))}))}))}));

    }
}