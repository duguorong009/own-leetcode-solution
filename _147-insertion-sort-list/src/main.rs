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
    let mut arr: Vec<i32> = vec![3, 5, 1, 3, 2, -1, 10, 4];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}

fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Collect the list node values into arr.
    let mut arr: Vec<i32> = vec![];
    let mut res: Option<Box<ListNode>> = head;
    while let Some(node) = res {
        arr.push(node.val);
        res = node.next;
    }

    // Insertion sort
    insertion_sort(&mut arr);

    // Build the result linked list.
    for &val in arr.iter().rev() {
        res = Some(Box::new(ListNode { val, next: res }));
    }
    res
}

fn insertion_sort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j >= 1 && arr[j] < arr[j - 1] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
