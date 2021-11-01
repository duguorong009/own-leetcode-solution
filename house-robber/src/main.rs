fn main() {
    let nums = vec![2, 7, 9, 3, 1];
    println!("{}", rob(nums));
}

fn rob(nums: Vec<i32>) -> i32 {
    // Essence of DP is to get the general formula that represents
    // ith state with past (i -1)th state and current state.
    if nums.len() == 1 {
        nums[0]
    } else if nums.len() == 2 {
        nums[0].max(nums[1])
    } else {
        let mut max_profit: Vec<i32> = vec![nums[0], nums[0].max(nums[1])];
        for i in 2..nums.len() {
            max_profit.push((max_profit[i - 1]).max(max_profit[i - 2] + nums[i]));
        }
        max_profit[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(rob(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(rob(nums), 12);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 7, 2, 3, 9];
        assert_eq!(rob(nums), 16);
    }
}
