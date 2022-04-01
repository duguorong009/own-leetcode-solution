fn main() {
    println!("Hello, world!");
}

fn reverse_string(s: &mut Vec<char>) {
    s.reverse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}