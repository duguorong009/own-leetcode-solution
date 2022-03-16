use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
    dp(n, k, target, &mut memo)
}

fn dp(n: i32, k: i32, target: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
    if let Some(&val) = memo.get(&(n, target)) {
        return val;
    }
    let res = if n == 0 {
        if target == 0 {
            1
        } else {
            0
        }
    } else {
        let mut sum = 0;
        for i in 1..=k {
            sum += dp(n - 1, k, target - i, memo);
            sum %= 1_000_000_007;
        }
        sum
    };
    memo.insert((n, target), res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(num_rolls_to_target(1, 6, 3), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(num_rolls_to_target(2, 6, 7), 6);
    }

    #[test]
    fn test_3() {
        assert_eq!(num_rolls_to_target(30, 30, 500), 222616187);
    }
}