fn main() {
    println!("Hello, world!");
}

fn int_to_roman(num: i32) -> String {

    const ROMANS: [(&str, &str, &str); 4] = [("I", "V", "X"), ("X", "L", "C"), ("C", "D", "M"), ("M", "", "")];

    let mut res: Vec<String> = vec![];

    // Disassemble the number.
    let mut digits: Vec<i32> = vec![];
    let mut num = num;
    while num != 0 {
        digits.push(num % 10);
        num = num / 10;
    }

    // Convert every digit to the corresponding roman.
    for (decimal, &digit) in digits.iter().enumerate() {
        let roman = match digit {
            0 => "".to_string(),
            1..=3 => ROMANS[decimal].0.repeat(digit as usize),
            4 => format!("{}{}", ROMANS[decimal].0, ROMANS[decimal].1),
            5 => ROMANS[decimal].1.to_string(),
            6..=8 => format!("{}{}", ROMANS[decimal].1, ROMANS[decimal].0.repeat(digit as usize - 5).as_str()),
            9 => format!("{}{}", ROMANS[decimal].0, ROMANS[decimal].2),
            _ => panic!("Invalid digit"),
        };

        res.push(roman);
    } 

    res.into_iter().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 3;
        assert_eq!(int_to_roman(num), "III".to_string());
    }

    #[test]
    fn test_2() {
        let num = 58;
        assert_eq!(int_to_roman(num), "LVIII".to_string());
    }

    #[test]
    fn test_3() {
        let num = 1994;
        assert_eq!(int_to_roman(num), "MCMXCIV".to_string());
    }

    #[test]
    fn test_4() {
        let num = 10;
        assert_eq!(int_to_roman(num), "X".to_string());
    }
}