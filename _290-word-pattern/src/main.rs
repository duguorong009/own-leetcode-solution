use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}
/// Given a `pattern` and a string `s`, find if `s` follows the same pattern.
/// Here, `follow` means a full match, such that there is a bijection between
/// a letter in `pattern` and a non-empty word in `s`.

fn word_pattern(pattern: String, s: String) -> bool {
    let chars: Vec<char> = pattern.chars().collect();
    let strings: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    if chars.len() != strings.len() {
        return false;
    }
    let mut hm1: HashMap<char, String> = HashMap::new();
    let mut hm2: HashMap<String, char> = HashMap::new();
    for i in 0..chars.len() {
        let ch = chars[i];
        let string = strings[i].clone();
        if let Some(s) = hm1.get(&ch) {
            if *s != string {
                return false;
            }
        } else {
            hm1.insert(ch, string.clone());
        }
        if let Some(c) = hm2.get(&string) {
            if *c != ch {
                return false;
            }
        } else {
            hm2.insert(string.clone(), ch);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let pattern = "abba".to_string(); 
        let s = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(pattern, s), true);
    }

    #[test]
    fn test_2() {
        let pattern = "abba".to_string(); 
        let s = "dog cat cat fish".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }

    #[test]
    fn test_3() {
        let pattern = "aaaa".to_string(); 
        let s = "dog cat cat dog".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }


    #[test]
    fn test_4() {
        let pattern = "abba".to_string(); 
        let s = "dog dog dog dog".to_string();
        assert_eq!(word_pattern(pattern, s), false);
    }
}