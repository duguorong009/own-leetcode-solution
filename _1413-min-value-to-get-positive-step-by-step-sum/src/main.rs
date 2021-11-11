fn main() {
    println!("Hello, world!");
}

fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    if nums.len() == 0 {
        return 1;
    }
    for i in 1..nums.len() {
        nums[i] += nums[i - 1];
    }
    1.max(1 - nums.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::min_start_value;

    #[test]
    fn test_1() {
        let nums = vec![-3, 2, -3, 4, 2];
        assert_eq!(min_start_value(nums), 5);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2];
        assert_eq!(min_start_value(nums), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, -2, -3];
        assert_eq!(min_start_value(nums), 5);
    }
}
