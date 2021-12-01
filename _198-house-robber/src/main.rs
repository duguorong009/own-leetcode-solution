fn main() {
    println!("Hello, world!");
}

fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut profit: Vec<i32> = vec![nums[0]];
    if nums[0] > nums[1] {
        profit.push(nums[0]);
    } else {
        profit.push(nums[1]);
    }

    for i in 2..n {
        profit.push(profit[i - 1].max(profit[i - 2] + nums[i]));
    }
    profit[n - 1]
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
}
