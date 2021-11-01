fn main() {
    println!("Hello, world!");
}

fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    let mut s_ptr = 0 as usize;
    let mut t_ptr = 0 as usize;
    while t_ptr < t.len() {
        if s[s_ptr] == t[t_ptr] {
            s_ptr += 1;
            if s_ptr == s.len() {
                break;
            }
        }
        t_ptr += 1;
    }
    s_ptr == s.len()
}

#[cfg(test)]
mod tests {
    use crate::is_subsequence;

    #[test]
    fn test_1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(is_subsequence(s, t), true);
    }

    #[test]
    fn test_2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert_eq!(is_subsequence(s, t), false);
    }
}
