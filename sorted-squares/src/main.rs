fn main() {
    let nums = vec![-5, -1, 3];
    println!("{:?}", sorted_squares(nums));
}

fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_by(|&a, &b| a.abs().cmp(&b.abs()));
    nums.iter().map(|&x| x.pow(2)).collect()
}

#[cfg(test)]
mod tests {
    use crate::sorted_squares;

    #[test]
    fn test_1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let expected_result = vec![0, 1, 9, 16, 100];
        assert_eq!(sorted_squares(nums), expected_result);
    }

    #[test]
    fn test_2() {
        let nums = vec![-7, -3, 2, 3, 11];
        let expected_result = vec![4, 9, 9, 49, 121];
        assert_eq!(sorted_squares(nums), expected_result);
    }
}
