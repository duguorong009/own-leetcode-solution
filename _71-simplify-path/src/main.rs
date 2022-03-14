fn main() {
    println!("Hello, world!");
}

fn simplify_path(path: String) -> String {
    let mut stack: Vec<&str> = vec![];
    let mut res = "".to_string();
    for s in path.split_terminator('/') {
        match s {
            ".." => {
                stack.pop();
            }
            "" | "." => {
                continue;
            }
            _ => {
                stack.push(s);
            }
        }
    }
    for s in stack {
        res += "/";
        res += s;
    }
    if res.is_empty() {
        res += "/";
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let path = "/home/".to_string();
        assert_eq!(simplify_path(path), "/home".to_string());
    }

    #[test]
    fn test_2() {
        let path = "/../".to_string();
        assert_eq!(simplify_path(path), "/".to_string());
    }

    #[test]
    fn test_3() {
        let path = "/home//foo/".to_string();
        assert_eq!(simplify_path(path), "/home/foo".to_string());
    }

    #[test]
    fn test_4() {
        let path = "////home////foo////../xy".to_string();
        assert_eq!(simplify_path(path), "/home/xy".to_string());
    }
}