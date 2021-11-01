fn main() {
    println!("Hello, world!");
}

fn is_valid(s: String) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    if s.len() < 2 {
        return false;
    }
    let mut stack: Vec<char> = vec![];
    for i in 0..s.len() {
        if s[i] == '(' || s[i] == '[' || s[i] == '{' {
            stack.push(s[i]);
        } else {
            if i == 0 {
                return false;
            } else {
                if let Some(stack_top) = stack.pop() {
                    if s[i] == ')' && stack_top != '(' {
                        return false;
                    } else if s[i] == ']' && stack_top != '[' {
                        return false;
                    } else if s[i] == '}' && stack_top != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn test_1() {
        let s = "()".to_string();
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn test_2() {
        let s = "()[]{}".to_string();
        assert_eq!(is_valid(s), true);
    }

    #[test]
    fn test_3() {
        let s = "(]".to_string();
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn test_4() {
        let s = "([)]".to_string();
        assert_eq!(is_valid(s), false);
    }

    #[test]
    fn test_5() {
        let s = "{[]}".to_string();
        assert_eq!(is_valid(s), true);
    }
}
