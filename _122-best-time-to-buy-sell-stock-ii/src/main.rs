fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let n = prices.len();
    for i in 0..n - 1 {
        if prices[i] < prices[i + 1] {
            profit += prices[i + 1] - prices[i];
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 4, 3, 6, 4];
        assert_eq!(max_profit(prices), 7);
    }

    #[test]
    fn test_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit(prices), 4);
    }

    #[test]
    fn test_3() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
