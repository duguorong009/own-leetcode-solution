fn main() {
    println!("Hello, world!");
}

fn is_match(s: String, p: String) -> bool {
    let n = s.len();
    let m = p.len();
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; m + 1]; n + 1];
    is_match_dp(n, m, &mut memo, &s, &p)
}

fn is_match_dp(
    n: usize,
    m: usize,
    memo: &mut Vec<Vec<Option<bool>>>,
    s: &[char],
    p: &[char],
) -> bool {
    if let Some(ans) = memo[n][m] {
        ans
    } else {
        let res = if n == 0 && m == 0 {
            true
        } else if n != 0 && m == 0 {
            false
        } else if n == 0 && m != 0 {
            if p[m - 1] == '*' {
                is_match_dp(n, m - 2, memo, s, p)
            } else {
                false
            }
        } else {
            if s[n - 1] == p[m - 1] {
                is_match_dp(n - 1, m - 1, memo, s, p)
            } else {
                match p[m - 1] {
                    '*' => match p[m - 2] {
                        '*' => false,
                        '.' => {
                            is_match_dp(n - 1, m, memo, s, p) || is_match_dp(n, m - 2, memo, s, p)
                        }
                        _ => {
                            if s[n - 1] != p[m - 2] {
                                is_match_dp(n, m - 2, memo, s, p)
                            } else {
                                is_match_dp(n - 1, m, memo, s, p)
                                    || is_match_dp(n, m - 2, memo, s, p)
                            }
                        }
                    },
                    '.' => is_match_dp(n - 1, m - 1, memo, s, p),
                    _ => false,
                }
            }
        };
        memo[n][m] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "aa".to_string();
        let p = "a".to_string();
        assert_eq!(is_match(s, p), false);
    }

    #[test]
    fn test_2() {
        let s = "aa".to_string();
        let p = "a*".to_string();
        assert_eq!(is_match(s, p), true);
    }

    #[test]
    fn test_3() {
        let s = "ab".to_string();
        let p = ".*".to_string();
        assert_eq!(is_match(s, p), true);
    }

    #[test]
    fn test_4() {
        let s = "aab".to_string();
        let p = "c*a*b".to_string();
        assert_eq!(is_match(s, p), true);
    }

    #[test]
    fn test_5() {
        let s = "mississippi".to_string();
        let p = "mis*is*p*.".to_string();
        assert_eq!(is_match(s, p), false);
    }
}
