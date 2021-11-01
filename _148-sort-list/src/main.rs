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

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    }
    let head = head;

    // Store node values into array.
    let mut temp: Option<Box<ListNode>> = head;
    let mut values: Vec<i32> = vec![];
    while temp != None {
        let list_node = temp.unwrap();
        values.push(list_node.val);
        temp = list_node.next;
    }

    // Sort the array in descending order.
    values.sort_by(|a, b| a.cmp(b).reverse());

    // Create the linked list with sorted array.
    let mut res: Option<Box<ListNode>> = None;
    for val in values.into_iter() {
        res = Some(Box::new(ListNode { val, next: res }));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tail = Some(Box::new(ListNode { val: 3, next: None }));
        let n2 = Some(Box::new(ListNode { val: 1, next: tail }));
        let n1 = Some(Box::new(ListNode { val: 2, next: n2 }));
        let head = Some(Box::new(ListNode { val: 4, next: n1 }));
        sort_list(head);
        assert_eq!(1 + 1, 1);
    }
}
