use std::vec;

fn main() {
    println!("Hello, world!");
}

fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut product: i32 = 0;
    for &num in &nums {
        product = product ^ num;
    }
    let diff = product & (-product);
    println!("{}", diff);
    let mut x = 0;
    for &num in &nums {
        if diff & num != 0 {
            x = x ^ num;
        }
    }
    vec![x, product ^ x]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        assert_eq!(single_number(nums), vec![3, 5]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 0];
        assert_eq!(single_number(nums), vec![-1, 0]);
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 1];
        assert_eq!(single_number(nums), vec![0, 1]);
    }
}
