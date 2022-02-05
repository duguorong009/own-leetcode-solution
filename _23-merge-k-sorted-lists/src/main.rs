use std::collections::VecDeque;

// Definition for singly-linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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

fn main() {
    println!("Hello, world!");
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }
    let mut queue: VecDeque<Option<Box<ListNode>>> = lists.into_iter().collect();
    while queue.len() > 1 {
        let merged_list = merge(queue.pop_front().unwrap(), queue.pop_front().unwrap());
        queue.push_back(merged_list);
    } 
    queue.pop_back().unwrap()
}

fn merge(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if a.is_none() && b.is_none() {
        return None;
    }

    if a.is_none() {
        return b;
    }

    if b.is_none() {
        return a;
    }

    let mut a = a.unwrap();
    let mut b  = b.unwrap();
    if a.val < b.val {
        a.next = merge(a.next.take(), Some(b));
        Some(a)
    } else {
        b.next = merge(Some(a), b.next.take());
        Some(b)
    }
}

