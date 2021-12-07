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

fn main() {
    println!("Hello, world!");
}

fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut res = 0;
    let mut temp: Option<Box<ListNode>> = head;
    while temp.is_some() {
        let node = temp.unwrap();
        res *= 2;
        if node.val == 1 {
            res += 1;
        }
        temp = node.next;
    }
    res
}
