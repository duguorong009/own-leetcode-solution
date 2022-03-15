fn main() {
    println!("Hello, world!");
}

fn min_remove_to_make_valid(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let mut res: Vec<char> = vec![];
    let mut paren: Vec<(usize, char)> = vec![];
    for i in 0..s.len() {
        if s[i] == '(' || s[i] == ')' {
            res.push('\0');
            paren.push((i, s[i]));
        } else {
            res.push(s[i]);
        }
    }

    // Update the "paren" to make valid parentheses.
    let valid_paren = make_valid_paren(&paren);

    for (i, ch) in valid_paren {
        res[i] = ch;
    }

    res.iter().filter(|v| v!= &&'\0').collect()
}

fn make_valid_paren(paren: &Vec<(usize, char)>) -> Vec<(usize, char)> {
    let mut res: Vec<(usize, char)> = vec![];
    let mut stack: Vec<(usize, char)> = vec![];
    for i in 0..paren.len() {
        let (_, ch) = paren[i];
        if ch == '(' {
            stack.push(paren[i]);
        } else {
            if !stack.is_empty() {
                res.push(stack.pop().unwrap());
                res.push(paren[i]);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "lee(t(c)o)de)".to_string();
        let expected = "lee(t(c)o)de".to_string();
        assert_eq!(min_remove_to_make_valid(s), expected);
    }

    #[test]
    fn test_2() {
        let s = "a)b(c)d".to_string();
        let expected = "ab(c)d".to_string();
        assert_eq!(min_remove_to_make_valid(s), expected);
    }

    #[test]
    fn test_3() {
        let s = "))((".to_string();
        assert_eq!(min_remove_to_make_valid(s), "".to_string());
    }

    #[test]
    fn test_make_valid_paren() {
        let paren = vec![(0, '('), (2, ')'), (5, ')')];
        let valid = make_valid_paren(&paren);
        assert_eq!(valid, vec![(0, '('), (2, ')')]);
    }
}