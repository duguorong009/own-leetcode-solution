fn main() {
    println!("{}", num_squares(25));
}

fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![std::usize::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut j = 1;
        while j * j <= i {
            dp[i] = dp[i].min(dp[i - j * j] + 1);
            j += 1;
        }
    }
    dp[n] as i32
}

#[cfg(test)]
mod tests {
    use crate::num_squares;

    #[test]
    fn test_1() {
        let n = 12;
        assert_eq!(num_squares(n), 3);
    }

    #[test]
    fn test_2() {
        let n = 13;
        assert_eq!(num_squares(n), 2);
    }
}
