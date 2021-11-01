fn main() {
    let nums = vec![1, 2, 3];
    println!("{}", rob(nums));
}

fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return nums[0];
    }
    let profit1 = get_max_profit(&nums[1..n]);
    let profit2 = get_max_profit(&nums[0..n - 1]);
    println!("{}, {}", profit1, profit2);

    profit1.max(profit2)
}

fn get_max_profit(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }
    let mut profit: Vec<i32> = vec![nums[0], nums[0].max(nums[1])];
    for i in 2..nums.len() {
        profit.push(profit[i - 1].max(profit[i - 2] + nums[i]));
    }

    profit[nums.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::rob;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![2, 3, 2];
        assert_eq!(rob(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        assert_eq!(rob(nums), 4);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 2, 3];
        assert_eq!(rob(nums), 3);
    }

    #[test]
    fn test_4() {
        let nums: Vec<i32> = vec![1];
        assert_eq!(rob(nums), 1);
    }
}
