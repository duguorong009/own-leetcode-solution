fn main() {
    println!("Hello, world!");
}

fn find_the_difference(s: String, t: String) -> char {
    let s: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = t.chars().collect();
    for elem in s {
        let index = t.iter().position(|e| *e == elem).unwrap();
        t.remove(index);
    } 
    t[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abcd".to_string();
        let t = "abcde".to_string();
        assert_eq!(find_the_difference(s, t), 'e');
    }

    #[test]
    fn test_2() {
        let s = "".to_string();
        let t = "y".to_string();
        assert_eq!(find_the_difference(s, t), 'y');
    }
}