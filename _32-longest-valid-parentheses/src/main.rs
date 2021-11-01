fn main() {
    println!("Hello, world!");
}

fn longest_valid_parentheses(s: String) -> i32 {
    let s = s.split("").collect::<Vec<&str>>();
    let mut open_symbols = 0;
    let mut close_symbols = 0;
    let mut cnt = 0;
    for symbol in s {
        println!("{}: {}, {}", symbol, open_symbols, close_symbols);
        if symbol == "(" {
            open_symbols += 1;
            if close_symbols != 0 {
                close_symbols = 0;
            }
        }
        if symbol == ")" {
            close_symbols += 1;
            if open_symbols > close_symbols {
                continue;
            } else if open_symbols == close_symbols {
                cnt += 2 * open_symbols;
                open_symbols = 0;
                close_symbols = 0;
            }
        }
    }
    // if close_symbols != 0 {
    //     cnt += 2 * close_symbols;
    // }
    cnt
}

#[cfg(test)]
mod tests {
    use crate::longest_valid_parentheses;

    #[test]
    fn test_1() {
        let s = "(()".to_string();
        assert_eq!(longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_2() {
        let s = ")()())".to_string();
        assert_eq!(longest_valid_parentheses(s), 4);
    }

    #[test]
    fn test_3() {
        let s = "".to_string();
        assert_eq!(longest_valid_parentheses(s), 0);
    }
    #[test]
    fn test_4() {
        let s = "()(())".to_string();
        assert_eq!(longest_valid_parentheses(s), 6);
    }

    #[test]
    fn test_5() {
        let s = "((()(()())))".to_string();
        assert_eq!(longest_valid_parentheses(s), 6);
    }

    #[test]
    fn test_6() {
        let s = "()(()".to_string();
        assert_eq!(longest_valid_parentheses(s), 2);
    }
}
