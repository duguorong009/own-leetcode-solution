fn main() {
    println!("Hello, world!");
}

// fn max_power(s: String) -> i32 {
//     let mut max_power = 1;
//     let s: Vec<&str> = s.split("").collect();
//     let mut power = 1;
//     let mut ch: &str = s[0];
//     for i in 1..s.len() {
//         if s[i] == ch {
//             power += 1;
//             max_power = max_power.max(power);
//         } else {
//             ch = s[i];
//             power = 1;
//         }
//     }

//     max_power
// }

fn max_power(s: String) -> i32 {
    let mut prev: (char, usize) = (' ', 0);
    let mut res = 0;
    for c in s.chars() {
        if c == prev.0 {
            prev.1 += 1;
        } else {
            prev = (c, 1);
        }
        res = res.max(prev.1);
    }
    res as i32
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
