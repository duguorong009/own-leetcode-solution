fn main() {
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(missing_number(nums), 8);
}

fn missing_number(nums: Vec<i32>) -> i32 {
    let mut xor = 0;
    let n = nums.len();
    for n in 0..=n {
        xor ^= n as i32;
    }
    for n in nums {
        xor ^= n;
    }
    xor
}

#[cfg(test)]
mod tests {
    use crate::missing_number;

    #[test]
    fn test_1() {
        let nums = vec![3, 0, 1];
        assert_eq!(missing_number(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1];
        assert_eq!(missing_number(nums), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(missing_number(nums), 8);
    }

    #[test]
    fn test_4() {
        let nums = vec![0];
        assert_eq!(missing_number(nums), 1);
    }
}
