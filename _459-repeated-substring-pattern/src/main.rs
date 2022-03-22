fn main() {
    println!("Hello, world!");
}

fn repeated_substring_pattern(s: String) -> bool {
    let mut t: String = "".to_string();
    let n = s.len();
    if n < 2 {
        return false;
    }
    t += &s;
    t += &s;
    t[1..2 * n - 1].contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(repeated_substring_pattern("abcabcabcabc".to_string()));
    }
}