fn main() {
    println!("Hello, world!");
}

fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();
    let n = word1.len();
    
    let mut alph1: Vec<i32> = vec![0; 26];
    let mut alph2: Vec<i32> = vec![0; 26];

    for i in 0..n {
        alph1[(word1[i] as u8 - b'a') as usize] += 1;
        alph2[(word2[i] as u8 - b'a') as usize] += 1;
    }

    for i in 0..26 {
        if (alph1[i] - alph2[i]).abs() > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_almost_equivalent_1() {
        let word1 = "aaaa".to_string();
        let word2 = "bccb".to_string();
        assert_eq!(check_almost_equivalent(word1, word2), false);
    }

    #[test]
    fn test_check_almost_equivalent_2() {
        let word1 = "abcdeef".to_string();
        let word2 = "abaaacc".to_string();
        assert_eq!(check_almost_equivalent(word1, word2), true);
    }

    #[test]
    fn test_check_almost_equivalent_3() {
        let word1 = "cccddabba".to_string();
        let word2 = "babababab".to_string();
        assert_eq!(check_almost_equivalent(word1, word2), true);
    }

    #[test]
    fn test_check_almost_equivalent_4() {
        let word1 = "eeee".to_string();
        let word2 = "zzzz".to_string();
        assert_eq!(check_almost_equivalent(word1, word2), false);
    }
}