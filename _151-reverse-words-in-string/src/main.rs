fn main() {
    println!("Hello, world!");
}

fn reverse_words(s: String) -> String {
    let s = s
        .trim_start()
        .trim_end()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    s.iter()
        .rev()
        .map(|val| *val)
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::reverse_words;

    #[test]
    fn test_1() {
        let s = "the sky is blue".to_string();
        assert_eq!(reverse_words(s), "blue is sky the".to_string());
    }

    #[test]
    fn test_2() {
        let s = "   hello world  ".to_string();
        assert_eq!(reverse_words(s), "world hello".to_string());
    }

    #[test]
    fn test_3() {
        let s = "a good    example".to_string();
        assert_eq!(reverse_words(s), "example good a".to_string());
    }

    #[test]
    fn test_4() {
        let s = "  Bob    Loves  Alice  ".to_string();
        assert_eq!(reverse_words(s), "Alice Loves Bob".to_string());
    }
}
