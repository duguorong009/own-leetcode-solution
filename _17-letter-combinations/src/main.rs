use std::{collections::HashMap, vec};

fn main() {
    println!("Hello, world!");
}

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let digits: Vec<char> = digits.chars().collect();
    let hm: HashMap<char, Vec<char>> = [
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]
    .iter()
    .map(|(d, v)| (*d, v.chars().collect()))
    .collect();
    let mut s: Vec<char> = vec![];
    let mut res: Vec<String> = vec![];
    dfs(&hm, &digits, &mut s, &mut res, 0);
    res
}

fn dfs(
    hm: &HashMap<char, Vec<char>>,
    digits: &[char],
    s: &mut Vec<char>,
    res: &mut Vec<String>,
    index: usize,
) {
    if index == digits.len() {
        res.push(s.iter().collect());
    } else {
        let d = digits[index];
        for &c in hm[&d].iter() {
            s.push(c);
            dfs(hm, digits, s, res, index + 1);
            s.pop();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::letter_combinations;

    #[test]
    fn test_1() {
        let digits = "23".to_string();
        let res_str = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        let res = res_str
            .into_iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>();
        assert_eq!(letter_combinations(digits), res)
    }

    #[test]
    fn test_2() {
        let digits = "".to_string();
        assert_eq!(letter_combinations(digits).len(), 0);
    }

    #[test]
    fn test_3() {
        let digits = "2".to_string();
        assert_eq!(
            letter_combinations(digits),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
