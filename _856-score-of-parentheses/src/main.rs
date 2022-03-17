fn main() {
    println!("Hello, world!");
}

fn score_of_parentheses(s: String) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for c in s.chars() {
        if c == '(' {
            stack.push(0);
        } else {
            let mut sum = 0;
            while let Some(last) = stack.pop() {
                if last != 0 {
                    sum += last;
                } else {
                    break;
                }
            }
            if sum == 0 {
                stack.push(1);
            } else {
                stack.push(2 * sum);
            }
        }
    }
    stack.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(score_of_parentheses("()".to_string()), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(score_of_parentheses("(())".to_string()), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(score_of_parentheses("()()".to_string()), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(score_of_parentheses("(()(()))".to_string()), 6);
    }
}