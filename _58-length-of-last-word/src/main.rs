fn main() {
    println!("Hello, world!");
}

fn length_of_last_word(s: String) -> i32 {
    s.split_ascii_whitespace().last().unwrap_or("").len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "Hello World".to_string();
        assert_eq!(length_of_last_word(s), 5);
    }

    #[test]
    fn test_2() {
        let s = "   fly me    to   the moon  ".to_string();
        assert_eq!(length_of_last_word(s), 4);
    }

    #[test]
    fn test_3() {
        let s = "luffy is still joyboy".to_string();
        assert_eq!(length_of_last_word(s), 6);
    }
}