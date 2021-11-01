fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut temp: Option<Box<ListNode>> = head;
    let mut vals: Vec<i32> = vec![];
    while temp != None {
        let node = temp.unwrap();
        vals.push(node.val);
        temp = node.next;
    }

    let mut l = 0;
    let mut r = vals.len() - 1;
    while l < r {
        if vals[l] != vals[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{is_palindrome, ListNode};

    #[test]
    fn test_1() {
        let tail = Some(Box::new(ListNode { val: 1, next: None }));
        let node2 = Some(Box::new(ListNode { val: 2, next: tail }));
        let node1 = Some(Box::new(ListNode {
            val: 2,
            next: node2,
        }));
        let head = Some(Box::new(ListNode {
            val: 1,
            next: node1,
        }));
        assert_eq!(is_palindrome(head), true);
    }

    #[test]
    fn test_2() {
        let node1 = Some(Box::new(ListNode { val: 2, next: None }));
        let head = Some(Box::new(ListNode {
            val: 1,
            next: node1,
        }));
        assert_eq!(is_palindrome(head), false);
    }
}
