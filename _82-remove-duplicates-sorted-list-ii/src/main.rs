fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[macro_export]
macro_rules! ListLink {
    ($v: expr) => {
        Some(Box::new(ListNode { val: $v, next: None }))
    };
    ($v: expr, $e: expr) => {
        Some(Box::new(ListNode { val: $v, next: $e }))
    };
}

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut temp: &Option<Box<ListNode>> = &head;
    let mut arr: Vec<i32> = vec![];
    while temp.is_some() {
        let node = temp.as_ref().unwrap();
        arr.push(node.val);
        temp = &node.next;
    }

    let sorted = remove_duplicates_arr(arr);

    let mut res: Option<Box<ListNode>> = None;
    for &val in sorted.iter().rev() {
        res = Some(Box::new(ListNode { val, next: res }));
    }
    res
}

// Note: "arr" should be sorted so that 
// the same values should be in sequence.
fn remove_duplicates_arr(arr: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let n = arr.len();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && arr[i] == arr[j] {
            j += 1;
        }
        if j == i + 1 {
            res.push(arr[i]);
        }
        i = j;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = ListLink!(1, ListLink!(2, ListLink!(3, ListLink!(3, ListLink!(4, ListLink!(4, ListLink!(5)))))));
        let expected = ListLink!(1, ListLink!(2, ListLink!(5)));
        assert_eq!(delete_duplicates(head), expected);
    }

    #[test]
    fn test_2() {
        let head = ListLink!(1, ListLink!(1, ListLink!(1, ListLink!(2, ListLink!(3)))));
        let expected = ListLink!(2, ListLink!(3));
        assert_eq!(delete_duplicates(head), expected);
    }

    #[test]
    fn test_remove_duplicate_arr() {
        let arr = vec![1, 1, 1, 2, 3];
        let expected = vec![2, 3];
        assert_eq!(remove_duplicates_arr(arr), expected);

        let arr = vec![1, 2, 3, 3, 4, 4, 5];
        let expected = vec![1, 2, 5];
        assert_eq!(remove_duplicates_arr(arr), expected);
    }
}