fn main() {
    println!("Hello, world!");
}

fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = 1;
        let y = 4;
        assert_eq!(hamming_distance(x, y), 2);
    }

    #[test]
    fn test_2() {
        let x = 3;
        let y = 1;
        assert_eq!(hamming_distance(x, y), 1);
    }
}
