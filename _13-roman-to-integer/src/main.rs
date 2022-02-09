fn main() {
    println!("Hello, world!");
}

fn roman_to_int(s: String) -> i32 {
    let mut res = 0;

    let s = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut id = 0;
    while id < n {
        let ch = s[id];
        let next_ch = if id + 1 < n { s[id + 1] } else { '_' };
        let addition = match ch {
            'I' => match next_ch {
                'V' => { id += 1; 4},
                'X' => { id += 1; 9},
                _ => 1,
            },
            'V' => 5, 
            'X' => match next_ch {
                'L' => { id += 1; 40 },
                'C' => { id += 1; 90 },
                _ => 10,
            }, 
            'L' => 50, 
            'C' => match next_ch {
                'D' => { id += 1; 400 },
                'M' => { id += 1; 900 },
                _ => 100,
            }
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid character"),
        };
        res += addition;
        id += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "III".to_string();
        assert_eq!(roman_to_int(s), 3);
    }

    #[test]
    fn test_2() {
        let s = "LVIII".to_string();
        assert_eq!(roman_to_int(s), 58);
    }

    #[test]
    fn test_3() {
        let s = "MCMXCIV".to_string();
        assert_eq!(roman_to_int(s), 1994);
    }
}