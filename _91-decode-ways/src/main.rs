fn main() {
    let s = "226".to_string();
    assert_eq!(num_decoding(s), 3);
}

fn num_decoding(s: String) -> i32 {
    let nums: Vec<u32> = s.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

    let n = nums.len();

    let mut dp: Vec<i32> = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = if nums[0] == 0 { 0 } else { 1 };
    for i in 2..=n {
        if nums[i - 1] >= 1 {
            dp[i] += dp[i - 1];
        }
        if nums[i - 2] * 10 + nums[i - 1] >= 10 && nums[i - 2] * 10 + nums[i - 1] <= 26 {
            dp[i] += dp[i - 2];
        }
    }
    println!("{:?}", dp);
    dp[n]
}

#[cfg(test)]
mod tests {
    use crate::num_decoding;

    #[test]
    fn test_1() {
        let s = "12".to_string();
        assert_eq!(num_decoding(s), 2);
    }

    #[test]
    fn test_2() {
        let s = "226".to_string();
        assert_eq!(num_decoding(s), 3);
    }

    #[test]
    fn test_3() {
        let s = "0".to_string();
        assert_eq!(num_decoding(s), 0);
    }

    #[test]
    fn test_4() {
        let s = "06".to_string();
        assert_eq!(num_decoding(s), 0);
    }
}
