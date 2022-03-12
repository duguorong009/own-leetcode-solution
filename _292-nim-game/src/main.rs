fn main() {
    println!("Hello, world!");
}

// If the "n" is the multiple of "4", then it is "lose".
// Otherwise, "win".
fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_win_nim() {
        assert!(!can_win_nim(4));
        assert!(can_win_nim(1));
        assert!(can_win_nim(2));
    }
}