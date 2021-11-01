fn main() {
    let nums = vec![2, 3, -2, 4];
    assert_eq!(max_product(nums), 6);
}

fn max_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    if n == 0 {
        return 0;
    }

    let mut min_val = nums[0];
    let mut max_val = nums[0];
    let mut result = nums[0];

    for i in 1..n {
        let temp = (nums[i] * max_val).max(nums[i] * min_val).max(nums[i]);
        min_val = (nums[i] * max_val).min(nums[i] * min_val).min(nums[i]);

        max_val = temp;
        result = result.max(max_val).max(min_val);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::max_product;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(max_product(nums), 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![-2, 0, -1];
        assert_eq!(max_product(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 0, 2, 0, 1, 0];
        assert_eq!(max_product(nums), 2);
    }
}
