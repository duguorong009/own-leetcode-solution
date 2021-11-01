fn main() {
    println!("Hello, world!");
}

fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .collect::<Vec<char>>();

    if s.len() < 2 {
        return true;
    }

    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if !s[l].to_lowercase().eq(s[r].to_lowercase()) {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn test_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(is_palindrome(s), true);
    }

    #[test]
    fn test_2() {
        let s = " race a car".to_string();
        assert_eq!(is_palindrome(s), false);
    }
    #[test]
    fn test_3() {
        let s = "".to_string();
        assert_eq!(is_palindrome(s), true);
    }
}
