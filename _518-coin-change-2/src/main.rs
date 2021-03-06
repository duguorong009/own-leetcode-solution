fn main() {
    let amount = 5;
    let coins = vec![1, 2, 5];
    assert_eq!(change(amount, coins), 4);
}

fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let n = amount + 1;
    let mut dp: Vec<i32> = vec![0; n];
    dp[0] = 1;
    for coin in coins {
        let mut sum = coin as usize;
        while sum <= amount {
            dp[sum] += dp[sum - coin as usize];
            sum += 1;
        }
    }
    dp[amount]
}

#[cfg(test)]
mod tests {
    use crate::change;

    #[test]
    fn test_1() {
        let amount = 5;
        let coins = vec![1, 2, 5];
        assert_eq!(change(amount, coins), 4);
    }

    #[test]
    fn test_2() {
        let amount = 3;
        let coins = vec![2];
        assert_eq!(change(amount, coins), 0);
    }

    #[test]
    fn test_3() {
        let amount = 10;
        let coins = vec![10];
        assert_eq!(change(amount, coins), 1);
    }
}
