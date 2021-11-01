fn main() {
    println!("Hello, world!");
}

fn my_atoi(s: String) -> i32 {
    let mut start = s.trim_start();
    let mut res: i32 = 0;
    let mut positive = true;
    if start.len() > 1 {
        let c = &start[0..1];
        match c {
            "+" => {
                start = &start[1..];
            }
            "-" => {
                start = &start[1..];
                positive = false;
            }
            _ => {
                if let Some(c) = c.chars().next() {
                    if !('0'..='9').contains(&c) {
                        return 0;
                    }
                }
            }
        }
    }
    for c in start.chars() {
        if ('0'..='9').contains(&c) {
            res = match res.checked_mul(10) {
                None => {
                    return overflow(positive);
                }
                Some(val) => val,
            };
            res = match res.checked_add((c as u8 - b'0') as i32) {
                None => {
                    return overflow(positive);
                }
                Some(val) => val,
            };
        } else {
            break;
        }
    }
    if !positive {
        res = match res.checked_mul(-1) {
            None => {
                return overflow(positive);
            }
            Some(val) => val,
        };
    }
    res
}

fn overflow(positive: bool) -> i32 {
    if positive {
        i32::MAX
    } else {
        i32::MIN
    }
}

#[cfg(test)]
mod tests {
    use crate::my_atoi;

    #[test]
    fn test_1() {
        let s = "42".to_string();
        assert_eq!(my_atoi(s), 42);
    }

    #[test]
    fn test_2() {
        let s = "     -42".to_string();
        assert_eq!(my_atoi(s), -42);
    }

    #[test]
    fn test_3() {
        let s = "4193 with words".to_string();
        assert_eq!(my_atoi(s), 4193);
    }

    #[test]
    fn test_4() {
        let s = "words and 987".to_string();
        assert_eq!(my_atoi(s), 0);
    }

    #[test]
    fn test_5() {
        let s = "-91283472332".to_string();
        assert_eq!(my_atoi(s), -2147483648);
    }

    #[test]
    fn test_6() {
        let s = "0032".to_string();
        assert_eq!(my_atoi(s), 32);
    }
}
