fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a = a.chars().map(|ch| ch.to_string()).collect::<Vec<String>>();
        let b = b.chars().map(|ch| ch.to_string()).collect::<Vec<String>>();

        Self::valid(&a, &b) || Self::valid(&b, &a)
    }

    fn valid(a: &Vec<String>, b: &Vec<String>) -> bool {
        let mut l = 0;
        let mut r = a.len() - 1;
        while l < r {
            if a[l] != b[r] {
                return Self::is_palindrome(a, l, r) || Self::is_palindrome(b, l, r);
            }
            l += 1;
            r -= 1;
        }

        true
    }

    fn is_palindrome(a: &Vec<String>, start: usize, end: usize) -> bool {
        let mut start = start;
        let mut end = end;
        while start < end {
            if a[start] != a[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = "x".to_string();
        let b = "y".to_string();
        assert!(Solution::check_palindrome_formation(a, b));
    }

    #[test]
    fn test_2() {
        let a = "xbdef".to_string();
        let b = "xecaf".to_string();
        assert!(!Solution::check_palindrome_formation(a, b));
    }

    #[test]
    fn test_3() {
        let a = "ulacfd".to_string();
        let b = "jizalu".to_string();
        assert!(Solution::check_palindrome_formation(a, b));
    }
}
