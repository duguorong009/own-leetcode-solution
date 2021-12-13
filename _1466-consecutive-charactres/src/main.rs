fn main() {
    println!("Hello, world!");
}

fn max_power(s: String) -> i32 {
    let mut max_power = 1;
    let s: Vec<&str> = s.split("").collect();
    let mut power = 1;
    let mut ch: &str = s[0];
    for i in 1..s.len() {
        if s[i] == ch {
            power += 1;
            max_power = max_power.max(power);
        } else {
            ch = s[i];
            power = 1;
        }
    }

    max_power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        assert_eq!(max_power(s), 2);
    }
    #[test]
    fn test_2() {
        let s = "abbcccddddeeeeedcba".to_string();
        assert_eq!(max_power(s), 5);
    }

    #[test]
    fn test_3() {
        let s = "triplepillooooow".to_string();
        assert_eq!(max_power(s), 5);
    }
    #[test]
    fn test_4() {
        let s = "hooraaaaaaaaaaay".to_string();
        assert_eq!(max_power(s), 11);
    }
    #[test]
    fn test_5() {
        let s = "tourist".to_string();
        assert_eq!(max_power(s), 1);
    }
}
