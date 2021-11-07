fn main() {
    let num1 = "123".to_string();
    let num2 = "456".to_string();
    assert_eq!(multiply(num1, num2), "56088".to_string());
}

fn multiply(num1: String, num2: String) -> String {
    let n1 = num1.len();
    let n2 = num2.len();
    let n3 = n1 + n2;
    let mut v1: Vec<i32> = vec![0; n1];
    let mut v2: Vec<i32> = vec![0; n2];
    let mut v3: Vec<i32> = vec![0; n3];
    let mut v4: Vec<i32> = vec![];
    for (i, c) in num1.char_indices() {
        v1[n1 - 1 - i] = (c as u8 - b'0') as i32;
    }
    for (i, c) in num2.char_indices() {
        v2[n2 - 1 - i] = (c as u8 - b'0') as i32;
    }
    for i in 0..n1 {
        for j in 0..n2 {
            v3[i + j] += v1[i] * v2[j];
        }
    }
    let mut carry = 0;
    for i in 0..n3 {
        v4.push((v3[i] + carry) % 10);
        carry = (v3[i] + carry) / 10;
    }
    if carry != 0 {
        v4.push(carry);
    }
    while let Some(0) = v4.last() {
        v4.pop();
    }
    v4.reverse();
    if v4.is_empty() {
        "0".to_string()
    } else {
        v4.into_iter().map(|x| (x as u8 + b'0') as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let num1 = "123".to_string();
        let num2 = "456".to_string();
        assert_eq!(multiply(num1, num2), "56088".to_string());
    }

    #[test]
    fn test_2() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();
        assert_eq!(multiply(num1, num2), "6".to_string());
    }

    #[test]
    fn test_3() {
        let num1 = "1234".to_string();
        let num2 = "1".to_string();
        assert_eq!(multiply(num1, num2), "1234".to_string());
    }
}
