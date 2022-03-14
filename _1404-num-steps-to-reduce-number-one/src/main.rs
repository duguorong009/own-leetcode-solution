fn main() {
    println!("Hello, world!");
}

fn num_steps(s: String) -> i32 {
    let mut count = 0;
    let mut s = s
        .split_terminator("")
        .filter(|v| v != &"")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    while s.len() != 1 {
        let n = s.len();
        if s[n - 1] == 0 {
            s.pop();
        } else {
            s[n - 1] = 0;
            let mut round = 1;
            for i in (0..n - 1).rev() {
                s[i] = s[i] + round;
                round = s[i] / 2;
                s[i] = s[i] % 2;
            }
            if round == 1 {
                s.insert(0, 1);
            }
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(num_steps("1101".to_string()), 6);
        assert_eq!(num_steps("10".to_string()), 1);
        assert_eq!(num_steps("1".to_string()), 0);
    }

    #[test]
    fn test_ex() {
        assert_eq!(num_steps("11001".to_string()), 8);
        assert_eq!(num_steps("1111011110000011100000110001011011110010111001010111110001".to_string()), 85);
    }
}