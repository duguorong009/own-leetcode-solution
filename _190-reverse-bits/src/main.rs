fn main() {
    println!("Hello, world!");
}

fn reverse_bits(x: u32) -> u32 {
    x.reverse_bits()
}

#[cfg(test)]
mod tests {
    use crate::reverse_bits;

    #[test]
    fn test_1() {
        let x: u32 = 0b00000010100101000001111010011100;
        assert_eq!(reverse_bits(x), 964176192);
    }

    #[test]
    fn test_2() {
        let x: u32 = 0b11111111111111111111111111111101;
        assert_eq!(reverse_bits(x), 3221225471);
    }
}
