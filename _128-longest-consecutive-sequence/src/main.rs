use std::collections::HashSet;
use std::iter::FromIterator;
fn main() {
    let arr = vec![2, 4, 3, 1, 7, 23, 1203, 24, 2];
    println!("{:?}", longest_consecutive(arr));
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let hs: HashSet<i32> = HashSet::from_iter(nums);
    let mut res = 0;
    for &x in &hs {
        if hs.contains(&(x - 1)) {
            continue;
        }
        let mut i = 1;
        while hs.contains(&(x + i)) {
            i += 1;
            res = res.max(i);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive;

    #[test]
    fn test_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(nums), 9);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 0, -1];
        assert_eq!(longest_consecutive(nums), 3);
    }
}
