fn main() {
    println!("Hello, world!");
}

fn is_same_after_reversals(num: i32) -> bool {
    if num == 0 {
        true
    } else {
        num % 10 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_after_reversals() {
        assert!(is_same_after_reversals(526));
        assert!(is_same_after_reversals(0));
        assert!(!is_same_after_reversals(1800));
    }
}