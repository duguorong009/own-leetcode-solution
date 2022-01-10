fn main() {
    println!("Hello, world!");
}

fn add_binary(a: String, b: String) -> String {
    let aa = i128::from_str_radix(&a, 2).unwrap_or(0);
    let bb = i128::from_str_radix(&b, 2).unwrap_or(0);
    format!("{:b}", aa + bb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!(add_binary(a, b), "100".to_string());
    }

    #[test]
    fn test_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!(add_binary(a, b), "10101".to_string());
    }
}
