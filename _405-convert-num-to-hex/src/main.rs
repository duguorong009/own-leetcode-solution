fn main() {
    println!("Hello, world!");
}

fn to_hex(num: i32) -> String {
    format!("{:x}", num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(to_hex(26), "1a".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(to_hex(-1), "ffffffff".to_string());
    }
}