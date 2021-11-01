fn main() {
    let arr: Vec<String> = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
    println!("{}", max_length(arr));
}

fn max_length(arr: Vec<String>) -> i32 {
    let arr_length = arr.len();
    arr.len() as i32 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr: Vec<String> = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
        assert_eq!(max_length(arr), 4);
    }

    #[test]
    fn test_2() {
        let arr: Vec<String> = vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string(),
        ];
        assert_eq!(max_length(arr), 6);
    }

    #[test]
    fn test_3() {
        let arr: Vec<String> = vec!["abcdefghijklmnopqrstuvwxyz".to_string()];
        assert_eq!(max_length(arr), 26);
    }
}
