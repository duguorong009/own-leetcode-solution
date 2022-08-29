fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut nums = nums;
        nums.sort();
        nums.dedup();

        if k == 0 || nums.len() == 1 {
            count = nums.len() as i32;
        } else {
            let mut min_val = nums[0];
            let mut i = 1;
            while i < nums.len() {
                if nums[i] - min_val > k {
                    min_val = nums[i];
                    count += 1;
                }
                i += 1;
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 6, 1, 2, 5];
        let k = 2;
        assert_eq!(2, Solution::partition_array(nums, k));
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let k = 1;
        assert_eq!(2, Solution::partition_array(nums, k));
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 2, 4, 5];
        let k = 0;
        assert_eq!(3, Solution::partition_array(nums, k));
    }
}
