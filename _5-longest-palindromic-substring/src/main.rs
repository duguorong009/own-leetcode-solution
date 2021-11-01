fn main() {
    println!("Hello, world!");
}

fn longest_palindrome(s: String) -> String {
    let n = s.len();
    if n == 0 {
        return "".to_string();
    }
    let s: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = 0;
    for i in 0..n {
        let mut left = i;
        let mut right = i;
        while right + 1 < n && s[right + 1] == s[left] {
            right += 1;
        }
        while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
            left -= 1;
            right += 1;
        }
        if right - left > end - start {
            start = left;
            end = right;
        }
    }
    s[start..=end].into_iter().collect::<String>()
}

fn check_palindrome(s: String) -> bool {
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::longest_palindrome;

    #[test]
    fn test_1() {
        let s = "babad".to_string();
        assert_eq!(longest_palindrome(s), "bab".to_string());
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
