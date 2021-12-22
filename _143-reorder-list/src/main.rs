use std::collections::VecDeque;

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

fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut p: Option<Box<ListNode>> = head.take();
    let mut deque: VecDeque<Option<Box<ListNode>>> = VecDeque::new();
    while let Some(mut n) = p {
        p = n.next.take();
        deque.push_back(Some(n));
    }
    let mut stack: Vec<Option<Box<ListNode>>> = vec![];
    let mut direction = true;
    while !deque.is_empty() {
        if direction {
            stack.push(deque.pop_front().unwrap());
        } else {
            stack.push(deque.pop_back().unwrap());
        }
        direction = !direction;
    }
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(last) = stack.pop() {
        let mut node = last.unwrap();
        node.next = prev;
        prev = Some(node);
    }
    *head = prev
}
