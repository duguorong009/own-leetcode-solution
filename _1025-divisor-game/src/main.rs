fn main() {
    println!("Hello, world!");
}

fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use crate::divisor_game;

    #[test]
    fn test_1() {
        let n = 2;
        assert_eq!(divisor_game(n), true);
    }

    #[test]
    fn test_2() {
        let n = 3;
        assert_eq!(divisor_game(n), false);
    }
}
