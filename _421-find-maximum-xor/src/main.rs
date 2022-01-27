use std::{collections::HashSet};

fn main() {
    let nums = vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70];
    assert_eq!(find_maximum_xor(nums), 127);
}

fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut max= 0;
    let mut mask = 0;
    for i in (0..30).rev() {
        mask |= 1 << i;
        let mut hs: HashSet<i32> = HashSet::new();
        for &x in &nums {
            hs.insert(x & mask);
        }
        let tmp = max | (1 << i);
        for &x in &hs {
            if hs.contains(&(tmp ^ x)) {
                max = tmp;
                break;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 10, 5, 25, 2, 8];
        assert_eq!(find_maximum_xor(nums), 28);
    }

    #[test]
    fn test_2() {
        let nums = vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70];
        assert_eq!(find_maximum_xor(nums), 127);
    }
}