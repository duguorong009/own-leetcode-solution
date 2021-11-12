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

fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    }

    let mut res = head;
    let mut vals: Vec<i32> = vec![];
    while res != None {
        let node = res.unwrap();
        let cur_node_val = node.val;
        if cur_node_val != val {
            vals.push(cur_node_val);
        }
        res = node.next;
    }

    for &val in vals.iter().rev() {
        res = Some(Box::new(ListNode { val, next: res }));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 6, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(remove_elements(head, 6), expected);
    }

    #[test]
    fn test_2() {
        let head: Option<Box<ListNode>> = None;
        assert_eq!(remove_elements(head, 1), None);
    }
    #[test]
    fn test_3() {
        let head = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 7,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode { val: 7, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(remove_elements(head, 7), None);
    }
}
