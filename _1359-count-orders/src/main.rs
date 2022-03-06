fn main() {
    println!("Hello, world!");
}

fn count_orders(n: i32) -> i32 {
    let n = n as i64;
    let mut res = 1;
    for i in 1..=n {
        res *= i * 2 - 1;
        res %= 1_000_000_007;
        res *= i;
        res %= 1_000_000_007;
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_orders(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(count_orders(2), 6);
    }

    #[test]
    fn test_3() {
        assert_eq!(count_orders(3), 90);
    }

    #[test]
    fn test_4() {
        assert_eq!(count_orders(8), 729647433);
    }
}