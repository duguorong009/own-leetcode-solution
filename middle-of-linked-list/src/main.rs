fn main() {
    let tail: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 5, next: None }));
    let item: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 4, next: tail }));
    let head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 3, next: item }));
    println!("{:?}", middle_node(head));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[warn(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Get the length of linked list.
    let mut list_length = 0;
    let mut temp_node: Option<Box<ListNode>> = head.clone();
    while temp_node != None {
        temp_node = temp_node.unwrap().next;
        list_length += 1;
    }

    // Calc length/2 value.
    let mut dest_length = list_length / 2;

    // Get that element by advancing length / 2 times from head.
    let mut result_node: Option<Box<ListNode>> = head;
    while dest_length != 0 {
        result_node = result_node.unwrap().next;
        dest_length -= 1;
    }
    result_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tail: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 5, next: None }));
        let item: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 4, next: tail }));
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 3, next: item }));
        let middle_nodel = middle_node(head);
        assert_eq!(
            middle_nodel,
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None }))
            }))
        )
    }
}
