fn main() {
    println!("Hello, world!");
}

fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left_odd = vec![0; n];
    let mut left_even = vec![0; n];
    let mut right_odd = vec![0; n];
    let mut right_even = vec![0; n];
    let mut odd_sum = 0;
    let mut even_sum = 0;
    for i in 0..n {
        left_odd[i] = odd_sum;
        left_even[i] = even_sum;
        if i % 2 == 0 {
            even_sum += nums[i];
        } else {
            odd_sum += nums[i];
        }
    }
    odd_sum = 0;
    even_sum = 0;
    for i in (0..n).rev() {
        right_odd[i] = odd_sum;
        right_even[i] = even_sum;
        if i % 2 == 0 {
            even_sum += nums[i];
        } else {
            odd_sum += nums[i];
        }
    }
    let mut res = 0;
    for i in 0..n {
        if left_odd[i] + right_even[i] == left_even[i] + right_odd[i] {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 1, 6, 4];
        assert_eq!(ways_to_make_fair(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1];
        assert_eq!(ways_to_make_fair(nums), 3);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3];
        assert_eq!(ways_to_make_fair(nums), 0);
    }
}