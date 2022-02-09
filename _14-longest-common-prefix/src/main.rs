fn main() {
    println!("Hello, world!");
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }
    let ss: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let n = ss.iter().map(|s| s.len()).min().unwrap();
    let mut prefix: Vec<char> = vec![];
    for i in 0..n {
        let c = ss[0][i];
        if ss.iter().all(|s| s[i] == c) {
            prefix.push(c);
        } else {
            break;
        }
    }
    prefix.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        assert_eq!(longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn test_2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(longest_common_prefix(strs), "".to_string());
    }
}