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

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }
    let mut p1 = l1.unwrap();
    let mut p2 = l2.unwrap();
    if p1.val < p2.val {
        p1.next = merge_two_lists(p1.next, Some(p2));
        Some(p1)
    } else {
        p2.next = merge_two_lists(Some(p1), p2.next);
        Some(p2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{merge_two_lists, ListNode};

    #[test]
    fn test_1() {
        let tail_1 = Some(Box::new(ListNode { val: 4, next: None }));
        let node2 = Some(Box::new(ListNode {
            val: 2,
            next: tail_1,
        }));
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: node2,
        }));

        let tail_2 = Some(Box::new(ListNode { val: 4, next: None }));
        let node_3 = Some(Box::new(ListNode {
            val: 3,
            next: tail_2,
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: node_3,
        }));

        let tail = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        }));
        let part_2 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: tail })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: part_2,
            })),
        }));

        let output = merge_two_lists(l1, l2);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_2() {
        let l1 = None;
        let l2 = None;
        assert_eq!(merge_two_lists(l1, l2), None);
    }

    #[test]
    fn test_3() {
        let l1: Option<Box<ListNode>> = None;
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));

        let expected = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(merge_two_lists(l1, l2), expected);
    }
}
