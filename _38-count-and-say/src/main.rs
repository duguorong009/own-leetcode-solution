fn main() {
    println!("{}", b'0');
}

fn count_and_say(n: i32) -> String {
    let mut res: Vec<String> = vec!["1".to_string()];
    let n = n as usize;
    for i in 1..n {
        let tmp = convert_string_to_count(res[i - 1].clone());
        res.push(tmp);
    }

    res[n - 1].clone()
}

fn convert_string_to_count(str: String) -> String {
    let mut res: Vec<char> = vec![];
    
    // Split the string to the arr of char.
    let str: Vec<char> = str.chars().collect();

    // Save the count and char in the result.
    let mut ch = str[0];
    let mut cnt: u8 = 0;
    for i in 0..str.len() {
        if ch == str[i] {
            cnt += 1;
        } else {
            res.push((cnt + b'0') as char);
            res.push(ch);
            ch = str[i];
            cnt = 1;
        }
    }
    res.push((cnt + b'0') as char);
    res.push(ch);

    res.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 1;
        assert_eq!(count_and_say(n), "1".to_string());
    }

    #[test]
    fn test_2() {
        let n = 4;
        assert_eq!(count_and_say(n), "1211".to_string());
    }

    #[test]
    fn test_convert_string_to_count() {
        let str = "3322251".to_string();
        assert_eq!(convert_string_to_count(str), "23321511".to_string());
    }
}