fn main() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(coin_change(coins, amount), 3);
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut coins = coins;
    coins.sort();
    let amount = amount as usize;
    let mut dp = vec![amount as i32 + 1; amount as usize + 1];
    dp[0] = 0;
    for i in 0..=amount {
        for j in 0..coins.len() {
            if coins[j] <= i as i32 {
                dp[i] = dp[i].min(1 + dp[i - coins[j] as usize]);
                println!("{}, {}: {:?}", i, j, dp);
            } else {
                break;
            }
        }
    }

    if dp[amount as usize] > amount as i32 {
        -1
    } else {
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::coin_change;

    #[test]
    fn test_1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(coin_change(coins, amount), 3);
    }

    #[test]
    fn test_2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(coin_change(coins, amount), -1);
    }

    #[test]
    fn test_3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(coin_change(coins, amount), 0);
    }

    #[test]
    fn test_4() {
        let coins = vec![1];
        let amount = 1;
        assert_eq!(coin_change(coins, amount), 1);
    }

    #[test]
    fn test_5() {
        let coins = vec![1];
        let amount = 2;
        assert_eq!(coin_change(coins, amount), 2);
    }
}
