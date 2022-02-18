fn main() {
    println!("Hello, world!");
}

fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k;
    let mut stack: Vec<char> = vec![];
    for c in num.chars() {
        while let Some(&top) = stack.last() {
            if k > 0 && top > c {
                stack.pop();
                k -= 1;
            } else {
                break;
            }
        }
        stack.push(c);
    }
    while k != 0 {
        stack.pop();
        k -= 1;
    }

    let res: String = stack.into_iter().skip_while(|&c| c == '0').collect();
    if res.is_empty() {
        "0".to_string()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = "1432219".to_string();
        let k = 3;
        assert_eq!(remove_kdigits(num, k), "1219".to_string());
    }

    #[test]
    fn test_2() {
        let num = "10200".to_string();
        let k = 1;
        assert_eq!(remove_kdigits(num, k), "200".to_string());
    }

    #[test]
    fn test_3() {
        let num = "10".to_string();
        let k = 2;
        assert_eq!(remove_kdigits(num, k), "0".to_string());
    }
}
