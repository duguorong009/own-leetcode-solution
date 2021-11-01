fn main() {
    println!("Hello, world!");
}

/*
dp[len][2]

Case 1: I have a stock on day i, dp[i][1] max of the below:
    - I bought it today
        dp[i - 2][0] - prices[i]
    - I am carry forwarding
        dp[i - 1][1]

Case 2: I have no stock on day i, dp[i][0] max of the below:
    -I sold it today
        dp[i - 1][1] + prices[i]
    - I am carry forwarding, doing nothing
        dp[i-1][0]
*/

fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n <= 1 {
        return 0;
    }

    if n == 2 && prices[1] > prices[0] {
        return prices[1] - prices[0];
    } else if n == 2 && prices[1] < prices[0] {
        return 0;
    }

    let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; n];
    dp[0][0] = 0;
    dp[0][1] = -prices[0];
    dp[1][0] = dp[0][0].max(dp[0][1] + prices[1]);
    dp[1][1] = dp[0][1].max(dp[0][0] - prices[1]);

    for i in 2..n {
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + prices[i]);
        dp[i][1] = dp[i - 1][1].max(dp[i - 2][0] - prices[i]);
    }
    // print!("{:?}", dp);
    dp[n - 1][0]
}

#[cfg(test)]
mod tests {
    use crate::max_profit;
    #[test]
    fn test_1() {
        let prices = vec![1, 2, 3, 0, 2];
        assert_eq!(max_profit(prices), 3);
    }

    #[test]
    fn test_2() {
        let prices = vec![1];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_3() {
        let prices = vec![1, 2, 4];
        assert_eq!(max_profit(prices), 3);
    }
}
