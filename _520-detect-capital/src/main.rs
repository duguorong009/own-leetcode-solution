fn main() {
    println!("Hello, world!");
}

fn detect_capital(word: String) -> bool {
    let word = word.chars().collect::<Vec<char>>();
    let mut capital_cnt = 0;
    for i in 0..word.len() {
        if word[i].is_uppercase() {
            capital_cnt += 1;
        }
    }
    if capital_cnt == word.len() || capital_cnt == 0 {
        return true;
    } 
    
    if capital_cnt == 1  && word[0].is_uppercase() {
        return true; 
    } 

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1(){
        assert_eq!(detect_capital("USA".to_string()), true);
    }

    #[test]
    fn test_2(){
        assert_eq!(detect_capital("FlaG".to_string()), false);
    }
}