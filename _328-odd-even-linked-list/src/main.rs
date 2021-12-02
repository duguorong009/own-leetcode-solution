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

fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut odd: Vec<Option<Box<ListNode>>> = vec![];
    let mut even: Vec<Option<Box<ListNode>>> = vec![];

    let mut i = 1;
    while let Some(mut node) = head {
        head = node.next.take();
        if i % 2 == 1 {
            odd.push(Some(node));
        } else {
            even.push(Some(node));
        }
        i += 1;
    }

    let mut res: Option<Box<ListNode>> = None;
    while let Some(mut node) = even.pop() {
        node.as_mut().unwrap().next = res;
        res = node;
    }

    while let Some(mut node) = odd.pop() {
        node.as_mut().unwrap().next = res;
        res = node;
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
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
        }));

        assert_eq!(odd_even_list(head), expected);
    }
}
