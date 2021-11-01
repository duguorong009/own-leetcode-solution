fn main() {
    let s = "hello".to_string();
    println!("{}", longest_palindrome(s));
}

fn longest_palindrome(s: String) -> String {
    "palindrome".to_string()
}

#[cfg(test)]
mod tests {
    use crate::longest_palindrome;

    #[test]
    fn test_1() {
        let s = "babad".to_string();
        assert_eq!(longest_palindrome(s), "aba".to_string());
    }

    #[test]
    fn test_2() {
        let s = "cbbd".to_string();
        assert_eq!(longest_palindrome(s), "bb".to_string());
    }

    #[test]
    fn test_3() {
        let s = "a".to_string();
        assert_eq!(longest_palindrome(s), "a".to_string());
    }

    #[test]
    fn test_4() {
        let s = "ac".to_string();
        assert_eq!(longest_palindrome(s), "a".to_string());
    }
}
