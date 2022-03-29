fn main() {
    println!("Hello, world!");
}

fn trailing_zeroes(n: i32) -> i32 {
    let mut result = 0;
    let mut n = n;
    while n != 0 {
        result += n / 5;
        n -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(trailing_zeroes(3), 0);
        assert_eq!(trailing_zeroes(5), 1);
        assert_eq!(trailing_zeroes(0), 0);
    }
}