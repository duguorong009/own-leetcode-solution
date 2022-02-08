fn main() {
    println!("Hello, world!");
}

// A number can be of the form 9x or 9x + k.
// For the first case, the answer is always 9.
// For the second case, it is always k.
fn add_digits(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    match num % 9 {
        0 => 9,
        k => k,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(add_digits(38), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(add_digits(0), 0);
    }
}