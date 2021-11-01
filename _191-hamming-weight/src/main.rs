fn main() {
    let n = 0b00000000000000000000000000001011;
    assert_eq!(hamming_weight(n), 3);
}

fn hamming_weight(n: u32) -> i32 {
    n.count_ones() as i32
}

#[cfg(test)]
mod tests {
    use crate::hamming_weight;

    #[test]
    fn test_1() {
        let n = 0b00000000000000000000000000001011;
        assert_eq!(hamming_weight(n), 3);
    }
    #[test]
    fn test_2() {
        let n = 0b00000000000000000000000010000000;
        assert_eq!(hamming_weight(n), 1);
    }
    #[test]
    fn test_3() {
        let n = 0b11111111111111111111111111111101;
        assert_eq!(hamming_weight(n), 31);
    }
}
