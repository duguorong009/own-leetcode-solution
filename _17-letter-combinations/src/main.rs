fn main() {
    println!("Hello, world!");
}

fn letter_combinations(digits: String) -> Vec<String> {
    vec![]
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
