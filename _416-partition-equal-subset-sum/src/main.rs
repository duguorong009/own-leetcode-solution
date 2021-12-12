fn main() {
    println!("Hello, world!");
}

fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let half = (sum / 2) as usize;
    let mut dp: Vec<bool> = vec![false; half as usize + 1];
    dp[0] = true;
    let mut max = 0;
    for x in nums {
        let j = x as usize;
        for i in (j..=half.min(max + j)).rev() {
            if dp[i - j] {
                dp[i] = true;
                max = max.max(i);
            }
        }
        if dp[half] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1, 5, 11, 5];
        assert_eq!(can_partition(nums), true);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 5];
        assert_eq!(can_partition(nums), false);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 5, 3];
        assert_eq!(can_partition(nums), false);
    }
}
