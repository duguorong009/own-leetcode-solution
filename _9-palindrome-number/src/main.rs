fn main() {
    println!("Hello, world!");
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut rev_x = x % 10;
    let mut temp = x / 10;
    while temp != 0 {
        rev_x = rev_x * 10 + temp % 10;
        temp = temp / 10;
    }
    x == rev_x
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test_1() {
        let x = 121;
        assert_eq!(is_palindrome(x), true);
    }

    #[test]
    fn test_2() {
        let x = -121;
        assert_eq!(is_palindrome(x), false);
    }

    #[test]
    fn test_3() {
        let x = 10;
        assert_eq!(is_palindrome(x), false);
    }

    #[test]
    fn test_4() {
        let x = -101;
        assert_eq!(is_palindrome(x), false);
    }
}
