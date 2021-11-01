fn main() {
    println!("Hello, world!");
}

fn is_power_of_two(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    n & (n - 1) == 0
}

#[cfg(test)]
mod tests {
    use crate::is_power_of_two;

    #[test]
    fn test_1() {
        let n = 16;
        assert_eq!(is_power_of_two(n), true);
    }

    #[test]
    fn test_2() {
        let n = 3;
        assert_eq!(is_power_of_two(n), false);
    }

    #[test]
    fn test_3() {
        let n = 1;
        assert_eq!(is_power_of_two(n), true);
    }

    #[test]
    fn test_4() {
        let n = 4;
        assert_eq!(is_power_of_two(n), true);
    }

    #[test]
    fn test_5() {
        let n = 5;
        assert_eq!(is_power_of_two(n), false);
    }

    #[test]
    fn test_6() {
        let n = 0;
        assert_eq!(is_power_of_two(n), false);
    }
}
