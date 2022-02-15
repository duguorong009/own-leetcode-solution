fn main() {
    println!("Hello, world!");
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut cur: String = "".to_string();
    dfs(n, n, &mut cur, &mut res);
    res
}

fn dfs(left: i32, right: i32, cur: &mut String, all: &mut Vec<String>) {
    if left == 0 && right == 0 {
        all.push(cur.to_string());
    } else {
        if left > 0 {
            cur.push('(');
            dfs(left - 1, right, cur, all);
            cur.pop();
        }
        if right > left {
            cur.push(')');
            dfs(left, right - 1, cur, all);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = vec!["((()))".to_string(), "(()())".to_string(), "(())()".to_string(), "()(())".to_string(), "()()()".to_string()];
        assert_eq!(generate_parentheses(3), expected);
    }

    #[test]
    fn test_2() {
        let expected = vec!["()".to_string()];
        assert_eq!(generate_parentheses(1), expected);
    }
}