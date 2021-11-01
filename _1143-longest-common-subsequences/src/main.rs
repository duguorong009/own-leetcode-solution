fn main() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    assert_eq!(longest_common_subsequences(text1, text2), 3);
}

fn longest_common_subsequences(text1: String, text2: String) -> i32 {
    let s1: Vec<char> = text1.chars().collect();
    let s2: Vec<char> = text2.chars().collect();
    let n1 = s1.len();
    let n2 = s2.len();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n2 + 1]; n1 + 1];
    for i in 0..n1 {
        for j in 0..n2 {
            if s1[i] == s2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    dp[n1][n2] as i32
}

#[cfg(test)]
mod tests {
    use crate::longest_common_subsequences;

    #[test]
    fn test_1() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        assert_eq!(longest_common_subsequences(text1, text2), 3);
    }

    #[test]
    fn test_2() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        assert_eq!(longest_common_subsequences(text1, text2), 3);
    }

    #[test]
    fn test_3() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        assert_eq!(longest_common_subsequences(text1, text2), 0);
    }

    #[test]
    fn test_4() {
        let text1 = "bl".to_string();
        let text2 = "yby".to_string();
        assert_eq!(longest_common_subsequences(text1, text2), 1);
    }

    #[test]
    fn test_5() {
        let text1 = "bsbininm".to_string();
        let text2 = "jmjkbkjkv".to_string();
        assert_eq!(longest_common_subsequences(text1, text2), 1);
    }
}
