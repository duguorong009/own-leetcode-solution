fn main() {
    println!("Hello, world!");
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let n = digits.len();
    let mut carry = 1;
    for i in (0..n).rev() {
        let sum = digits[i] + carry;
        res.push(sum % 10);
        carry = sum / 10; 
    }
    if carry != 0 {
        res.push(carry);
    }
    res.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let digits = vec![1, 2, 3, 4];
        assert_eq!(plus_one(digits), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_2() {
        let digits = vec![9, 9, 9, 9];
        assert_eq!(plus_one(digits), vec![1, 0, 0, 0, 0]);
    }
}