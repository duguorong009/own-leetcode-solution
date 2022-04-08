use std::collections::VecDeque;
use std::vec;

fn main() {
    println!("Hello, world!");
}

fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let n = changed.len();
    let mut changed = changed;
    changed.sort_unstable();
    let mut queue: VecDeque<i32> = VecDeque::new();
    for i in 0..n {
        if !queue.is_empty() && changed[i] == *queue.front().unwrap() {
            queue.pop_front();
        } else {
            queue.push_back(changed[i] * 2);
            res.push(changed[i]);
        }
    }
    if queue.is_empty() {
        res
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let changed = vec![1, 3, 4, 2, 6, 8];
        assert_eq!(find_original_array(changed), vec![1, 3, 4]);
    }

    #[test]
    fn test_2() {
        let changed = vec![6, 3, 0, 1];
        assert_eq!(find_original_array(changed), vec![]);
    }

    #[test]
    fn test_3() {
        let changed = vec![1];
        assert_eq!(find_original_array(changed), vec![]);
    }
}