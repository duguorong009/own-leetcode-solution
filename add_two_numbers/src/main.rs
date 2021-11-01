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

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut round: i32 = 0;
    let mut result: Option<Box<ListNode>> = None;
    let mut l1_node_box = l1.unwrap();
    let mut l2_node_box = l2.unwrap();
    loop {
        let sum: i32 = l1_node_box.val + l2_node_box.val + round;
        let new_val = sum % 10 as i32;
        round = sum / 10 as i32;

        result = Some(Box::new(ListNode {
            val: new_val,
            next: result,
        }));

        if l1_node_box.next == None && l2_node_box.next != None {
            l1_node_box.next = Some(Box::new(ListNode::new(0)));
        } else if l1_node_box.next != None && l2_node_box.next == None {
            l2_node_box.next = Some(Box::new(ListNode::new(0)));
        } else if l1_node_box.next == None && l2_node_box.next == None {
            if round == 0_i32 {
                break;
            } else {
                l1_node_box.next = Some(Box::new(ListNode::new(0)));
                l2_node_box.next = Some(Box::new(ListNode::new(0)));
            }
        }

        l1_node_box = l1_node_box.next.unwrap();
        l2_node_box = l2_node_box.next.unwrap();
    }

    reverse_linked_list(result)
}

fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut origin: Option<Box<ListNode>> = head;
    let mut result: Option<Box<ListNode>> = None;
    loop {
        if origin == None {
            break;
        } else {
            let node = origin.unwrap();
            result = Some(Box::new(ListNode {
                val: node.val,
                next: result,
            }));
            origin = node.next;
        }
    }
    result
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode::new(6))),
    }));
    // println!("{:?}", reverse_linked_list(l1));
    let l2 = Some(Box::new(ListNode::new(8)));
    let result = add_two_numbers(l1, l2);
    println!("{:?}", result);
}
