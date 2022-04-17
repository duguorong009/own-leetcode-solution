fn main() {
    println!("Hello, world!");
}

fn square_is_white(coordinates: String) -> bool {
    let coords_sum: u32 = coordinates
        .chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| (c as u8 - b'a') as u32)
        }).sum();

    coords_sum % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!square_is_white("a1".to_string()));
        assert!(square_is_white("h3".to_string()));
        assert!(!square_is_white("c7".to_string()));
    }
}
