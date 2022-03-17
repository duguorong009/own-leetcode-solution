fn main() {
    println!("Hello, world!");
}

fn check_if_pangram(sentence: String) -> bool {
    let mut alphabets: Vec<i32> = vec![0; 26];
    for ch in sentence.chars() {
        alphabets[(ch as u8 - b'a') as usize] += 1;
        if !alphabets.contains(&0) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_pangram() {
        assert!(check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()));
        assert!(!check_if_pangram("leetcode".to_string()));
    }
}