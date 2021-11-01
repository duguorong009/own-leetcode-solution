fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[warn(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn remove_nth_from_the_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // // Get the length/count of linked list.
    // let mut list_length = 1;
    // let mut temp_node: Option<Box<ListNode>> = head.clone();
    // while temp_node != None {
    //     temp_node = temp_node.unwrap().next;
    //     list_length += 1;
    // }

    // // Remove the nth node from the end of linked list.
    // if list_length == n {
    //     None
    // } else {
    //     let mut node: Option<Box<ListNode>> = head.clone();
    //     let mut index = 1;
    //     while node != None {
    //         if index == list_length - n {
    //             node = node.unwrap().next.unwrap().next;
    //         } else {
    //             node = node.unwrap().next;
    //         }
    //         index += 1;
    //     }
    //     head
    // }

    // convert linked list into array or vector of ListNode.
    // Save the nodes of linked list into vector.
    let mut list_values: Vec<i32> = vec![];
    let mut temp_node: Option<Box<ListNode>> = head.clone();
    while temp_node != None {
        list_values.push(temp_node.as_ref().unwrap().as_ref().val);
        temp_node = temp_node.unwrap().next;
    }

    // Reverse-loop the vector and create new linked-list.
    let mut res: Option<Box<ListNode>> = None;
    for (id, val) in list_values.into_iter().rev().enumerate() {
        if id == (n - 1) as usize {
            continue;
        } else {
            res = Some(Box::new(ListNode {
                val: val,
                next: res,
            }));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let tail: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 5, next: None }));
        let item4: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 4, next: tail }));
        let item3: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 3,
            next: item4,
        }));
        let item2: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 2,
            next: item3,
        }));
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: item2,
        }));

        let updated_list = remove_nth_from_the_end(head, 2);

        let tail: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 5, next: None }));
        let item3: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 3, next: tail }));
        let item2: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 2,
            next: item3,
        }));
        let expected_head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: item2,
        }));
        assert_eq!(updated_list, expected_head);
    }

    #[test]
    fn test_2() {
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 1, next: None }));

        let updated_list = remove_nth_from_the_end(head, 1);
        assert_eq!(updated_list, None);
    }

    #[test]
    fn test_3() {
        let tail: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 2, next: None }));
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 1, next: tail }));

        let updated_list = remove_nth_from_the_end(head, 1);
        assert_eq!(
            updated_list,
            Some(Box::new(ListNode { val: 1, next: None }))
        );
    }
}
