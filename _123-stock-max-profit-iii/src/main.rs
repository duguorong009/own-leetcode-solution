fn main() {
    let prices = vec![3, 3, 5, 0, 7, 1, 4];
    assert_eq!(max_profit(prices), 6);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    // let n = prices.len();
    // if n == 0 {
    //     return 0;
    // }

    // let mut fb = i32::MIN;
    // let mut sb = i32::MIN;
    // let mut fs = 0;
    // let mut ss = 0;

    // for i in 0..n {
    //     fb = fb.max(-prices[i]);
    //     fs = fs.max(fb + prices[i]);
    //     sb = sb.max(fs - prices[i]);
    //     ss = ss.max(sb + prices[i]);
    // }
    // ss

    let mut t1_cost = std::i32::MAX;
    let mut t2_cost = std::i32::MAX;
    let mut t1_profit = 0;
    let mut t2_profit = 0;
    for x in prices {
        t1_cost = t1_cost.min(x);
        t1_profit = t1_profit.max(x - t1_cost);
        t2_cost = t2_cost.min(x - t1_profit);
        t2_profit = t2_profit.max(x - t2_cost);
    }
    t2_profit
}

#[cfg(test)]
mod tests {
    use crate::max_profit;

    #[test]
    fn test_1() {
        let prices = vec![3, 3, 5, 0, 0, 1, 4];
        assert_eq!(max_profit(prices), 6);
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

    #[test]
    fn test_4() {
        let prices = vec![1];
        assert_eq!(max_profit(prices), 0);
    }
}
