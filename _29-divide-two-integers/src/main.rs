fn main() {
    println!("Hello, world!");
}

fn divide(dividend: i32, divisor: i32) -> i32 {
    if let Some(res) = dividend.checked_div(divisor) {
        res
    } else {
        i32::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let dividend = 10;
        let divisor = 3;
        assert_eq!(divide(dividend, divisor), 3);
    }

    #[test]
    fn test_2() {
        let dividend = 7;
        let divisor = -3;
        assert_eq!(divide(dividend, divisor), -2);
    }

    #[test]
    fn test_3() {
        let dividend = -2147483648;
        let divisor = -1;
        assert_eq!(divide(dividend, divisor), 2147483647);
    }
}