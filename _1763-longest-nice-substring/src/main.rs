use std::{collections::HashSet};
fn main() {
    println!("Hello, world!");
}

fn longest_nice_substring(s: String) -> String {
    let mut max_sub: String = "".to_string();
    let n = s.len();
    let s: Vec<char> = s.chars().collect();
    let s_lower: Vec<char> = s.iter()
        .map(|&ch| ch.to_ascii_lowercase())
        .collect();
    for i in 0..n {
        for j in (i..n).rev() {
            let lower_cnt = count_unique_chars(&s_lower[i..=j]);
            let lower_upper_cnt = count_unique_chars(&s[i..=j]);
            if lower_upper_cnt == 2 * lower_cnt {
                if j - i > max_sub.len() {
                    max_sub = s[i..=j].iter().collect();
                }
            }
        }
    }
    max_sub
}

fn count_unique_chars(arr: &[char]) -> i32 {
    let mut hs: HashSet<char> = HashSet::new();
    for &ch in arr {
        hs.insert(ch);
    }
    hs.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(longest_nice_substring("YazaAay".to_string()), "aAa".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(longest_nice_substring("Bb".to_string()), "Bb".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(longest_nice_substring("c".to_string()), "".to_string());
    }
}