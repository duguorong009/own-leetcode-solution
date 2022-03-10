use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut a = vec![false; n + 1];
    let hs: HashSet<String> = HashSet::from_iter(word_dict);
    a[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if a[j] && hs.contains(&s[j..i]) {
                a[i] = true;
                break;
            }
        }
    }
    a[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let word_dict = vec!["leet".to_string(), "code".to_string()];
        assert_eq!(word_break(s, word_dict), true);
    }

    #[test]
    fn test_2() {
        let s = "applepenapple".to_string();
        let word_dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(word_break(s, word_dict), true);
    }

    #[test]
    fn test_3() {
        let s = "catsandog".to_string();
        let word_dict = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ];
        assert_eq!(word_break(s, word_dict), false);
    }
}
