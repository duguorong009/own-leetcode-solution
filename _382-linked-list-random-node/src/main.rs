use rand::prelude::*;

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

struct Solution {
    head: Option<Box<ListNode>>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let rng = thread_rng();
        Solution { head, rng }
    }

    fn get_random(&mut self) -> i32 {
        let mut cur = &self.head;
        let mut res = 0;
        let mut count = 0;
        while let Some(node) = cur {
            let val = node.val;
            count += 1;
            if self.rng.gen_range(0..count) == 0 {
                res = val;
            }
            cur = &node.next;
        }
        res
    }
}
