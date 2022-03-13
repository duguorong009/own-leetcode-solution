fn main() {
    println!("Hello, world!");
}

fn common_chars(words: Vec<String>) -> Vec<String> {
    let n = words.len();
    let mut counts: Vec<Vec<usize>> = vec![vec![0; 256]; n];
    for i in 0..n {
        let w = &words[i];
        for c in w.chars() {
            counts[i][c as usize] += 1;
        }
    }

    let mut res: Vec<String> = vec![];
    for i in 0..26 {
        let c: u8 = b'a' + i;
        let mut min = usize::MAX;
        for j in 0..n {
            let count = counts[j][c as usize];
            min = usize::min(count, min);
        }
        for _ in 0..min {
            res.push(format!("{}", c as char));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["bella".to_string(), "label".to_string(), "roller".to_string()];
        let expected = vec!["e".to_string(), "l".to_string(), "l".to_string()];
        assert_eq!(common_chars(words), expected);
    }

    #[test]
    fn test_2() {
        let words = vec!["cool".to_string(), "lock".to_string(), "cook".to_string()];
        let expected = vec!["c".to_string(), "o".to_string()];
        assert_eq!(common_chars(words), expected);
    }
}