fn main() {
    println!("{}", reverse(123456789));
}

fn reverse(x: i32) -> i32 {
    let x_str = x.abs().to_string().chars().rev().collect::<String>();
    if let Ok(y) = x_str.parse::<i32>() {
        x.signum() * y
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse;

    #[test]
    fn test_1() {
        let x = 123;
        assert_eq!(reverse(x), 321_i32);
    }

    #[test]
    fn test_2() {
        let x = -123;
        assert_eq!(reverse(x), -321_i32);
    }

    #[test]
    fn test_3() {
        let x = 120;
        assert_eq!(reverse(x), 21_i32);
    }

    #[test]
    fn test_4() {
        let x = 0;
        assert_eq!(reverse(x), 0_i32);
    }

    #[test]
    fn test_5() {
        let x = 1534236469;
        assert_eq!(reverse(x), 0_i32);
    }
}
