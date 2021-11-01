use std::collections::HashMap;

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp: HashMap<(usize, i64), usize> = HashMap::new();
    let mut res = 0;
    for i in 0..n {
        for j in 0..i {
            let diff = nums[i] as i64 - nums[j] as i64;
            let prev = *dp.entry((j, diff)).or_insert(1);
            res += prev - 1;
            *dp.entry((i, diff)).or_insert(1) += prev;
        }
    }
    println!("{:?}", dp);
    res as i32
}

fn main() {
    let nums = vec![2, 4, 6, 8, 10];
    println!("{}", number_of_arithmetic_slices(nums));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![2, 4, 6, 8, 10];
        assert_eq!(number_of_arithmetic_slices(nums), 7);
    }

    #[test]
    fn test_2() {
        let nums = vec![7, 7, 7, 7, 7];
        assert_eq!(number_of_arithmetic_slices(nums), 16);
    }
}
