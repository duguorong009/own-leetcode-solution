fn main() {
    let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{}", max_sub_array(nums));
}

// Example 1.
// Arr       5  4  -1   7   8
// prev  0   5  9   8  15  23
// max -Inf  5  9   9  15  23

// Example 2.
// Arr       -2   1   -3   4  -1  2  1  -5  4
// prev  0   -2   1   -2   4   3  5  6   1  5  (prev = num.max(num + prev))
// max  -Inf -2   1    1   4   4  5  6   6  6  (max = max.max(prev))
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut max_sum = std::i32::MIN;
    for i in 0..nums.len() {
        prev = nums[i].max(prev + nums[i]);
        max_sum = max_sum.max(prev);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use crate::max_sub_array;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1];
        assert_eq!(max_sub_array(nums), 1);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![5, 4, -1, 7, 8];
        assert_eq!(max_sub_array(nums), 23);
    }
}
