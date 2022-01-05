fn main() {
    println!("Hello, world!");
}

fn partition(s: String) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = vec![];
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut indexes: Vec<(usize, usize)> = vec![];
    dfs(0, &mut indexes, &mut res, &s, n);
    res
}

fn dfs(
    start: usize,
    indexes: &mut Vec<(usize, usize)>,
    strings: &mut Vec<Vec<String>>,
    s: &[char],
    n: usize,
) {
    if start == n {
        let mut partition: Vec<String> = vec![];
        for &(l, r) in indexes.iter() {
            partition.push(s[l..r].iter().collect());
        }
        strings.push(partition);
    }
    for end in start + 1..=n {
        if is_palindrome(&s[start..end]) {
            indexes.push((start, end));
            dfs(end, indexes, strings, s, n);
            indexes.pop();
        }
    }
}

fn is_palindrome(s: &[char]) -> bool {
    let n = s.len();
    for i in 0..n {
        let j = n - 1 - i;
        if s[i] != s[j] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            partition("aab".to_string()),
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(partition("a".to_string()), vec![vec!["a".to_string()]]);
    }
}
