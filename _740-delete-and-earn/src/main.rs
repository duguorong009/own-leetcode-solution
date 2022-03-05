fn main() {
    println!("Hello, world!");
}

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let n = 10001;
    let mut sum: Vec<i32> = vec![0; n];
    let mut dp: Vec<i32> = vec![0; n];
    for x in nums {
        sum[x as usize] += x;
    }
    dp[1] = sum[1];
    for i in 2..n {
        dp[i] = i32::max(sum[i] + dp[i - 2], dp[i - 1]);
    }
    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 2];
        assert_eq!(delete_and_earn(nums), 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(delete_and_earn(nums), 9);
    }
}