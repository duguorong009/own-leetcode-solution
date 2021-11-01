fn main() {
    let nums = vec![1, 2, 3];
    let target = 4;
    assert_eq!(combination_sum4(nums, target), 7);
}

fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let mut dp: Vec<i32> = vec![0; target + 1];
    dp[0] = 1;
    for i in 1..target + 1 {
        for &num in nums.iter() {
            dp[i] += if i as i32 - num >= 0 {
                dp[i - num as usize]
            } else {
                0
            };
        }
    }
    dp[target]
}

#[cfg(test)]
mod tests {
    use crate::combination_sum4;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let target = 4;
        assert_eq!(combination_sum4(nums, target), 7);
    }

    #[test]
    fn test_2() {
        let nums = vec![9];
        let target = 3;
        assert_eq!(combination_sum4(nums, target), 0);
    }
}
