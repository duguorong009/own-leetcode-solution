fn main() {
    println!("Hello, world!");
}

fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    let n = n
        .to_string()
        .split("")
        .map(|v| v.to_string())
        .filter(|v| *v != "".to_string())
        .collect::<Vec<String>>();
    let n_len = n.len();

    let mut res = 0_i32;

    // Count numbers that have less digits than "n".
    for how_many_digits in 1..n_len {
        res += digits.len().pow(how_many_digits as u32) as i32;
    }

    // Count numbers that has same digits as "n".
    for i in 0..n_len {
        let suitable_digits = digits
            .iter()
            .filter(|d| (**d).parse::<i32>().unwrap() < n[i].parse::<i32>().unwrap())
            .map(|v| v.clone())
            .collect::<Vec<String>>();

        res += (suitable_digits.len() * (digits.len().pow((n_len - i - 1) as u32))) as i32;

        if !digits.contains(&n[i]) {
            res += if i == n_len { 1 } else { 0 };
            break;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let digits = vec![
            "1".to_string(),
            "3".to_string(),
            "5".to_string(),
            "7".to_string(),
        ];
        let n = 100;
        assert_eq!(at_most_n_given_digit_set(digits, n), 20);
    }

    #[test]
    fn test_2() {
        let digits = vec!["1".to_string(), "4".to_string(), "9".to_string()];
        let n = 1_000_000_000;
        assert_eq!(at_most_n_given_digit_set(digits, n), 29523);
    }

    #[test]
    fn test_3() {
        let digits = vec!["7".to_string()];
        let n = 8;
        assert_eq!(at_most_n_given_digit_set(digits, n), 1);
    }
}
