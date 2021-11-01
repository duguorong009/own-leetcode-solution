fn main() {
    println!("Hello, world!");
}
fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut buy = prices[0];
    let mut profit = 0;

    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        }
        if prices[i] - buy > profit {
            profit = prices[i] - buy;
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    use crate::max_profit;

    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
