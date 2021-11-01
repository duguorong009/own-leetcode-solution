fn main() {
    println!("Hello, world!");
}

fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut res: i32 = 0;
    let mut change: i32 = -1;

    let mut left = left;
    let mut right = right;
    while change != 0 {
        change = range_bitwise_and_unit(left, right);
        res += change;
        left -= change;
        right -= change;
    }

    res
}

fn range_bitwise_and_unit(left: i32, right: i32) -> i32 {
    if left == 0 || right == 0 {
        return 0;
    }
    if left == right {
        return left;
    }

    let left_log_2 = (left as f64).log2().floor() as u32;
    let right_log_2 = (right as f64).log2().floor() as u32;

    if left_log_2 != right_log_2 {
        0
    } else {
        2_i32.pow(left_log_2)
    }
}

#[cfg(test)]
mod tests {
    use crate::range_bitwise_and;

    #[test]
    fn test_1() {
        let left = 5;
        let right = 7;
        assert_eq!(range_bitwise_and(left, right), 4);
    }

    #[test]
    fn test_2() {
        let left = 0;
        let right = 0;
        assert_eq!(range_bitwise_and(left, right), 0);
    }

    #[test]
    fn test_3() {
        let left = 1;
        let right = 2147483647;
        assert_eq!(range_bitwise_and(left, right), 0);
    }

    #[test]
    fn test_4() {
        let left = 3;
        let right = 3;
        assert_eq!(range_bitwise_and(left, right), 3);
    }

    #[test]
    fn test_5() {
        let left = 6;
        let right = 7;
        assert_eq!(range_bitwise_and(left, right), 6);
    }
}
