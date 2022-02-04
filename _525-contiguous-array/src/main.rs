use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut res: usize = 0;
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut diff = 0;
    let n = nums.len();
    hm.entry(0).or_default();
    for i in 0..n {
        if nums[i] == 1 {
            diff += 1;
        } else {
            diff -= 1;
        }
        let j = *hm.entry(diff).or_insert(i + 1);
        res = usize::max(i + 1 - j, res);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![0, 1];
        assert_eq!(find_max_length(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1, 0];
        assert_eq!(find_max_length(nums), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 1, 0, 1];
        assert_eq!(find_max_length(nums), 4);
    }

    #[test]
    fn test_4() {
        let nums = vec![0, 0, 1, 1];
        assert_eq!(find_max_length(nums), 4);
    }
}