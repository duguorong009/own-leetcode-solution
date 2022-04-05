fn main() {
    println!("Hello, world!");
}

fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let size = 2 * k as usize + 1;
    if n < size {
        return vec![-1; n];
    }

    let mut averages: Vec<i32> = vec![-1; n];
    let k = k as usize;
    let mut sum = nums[0..size].iter().sum::<i32>();
    // Update the averages for range [k, n - k - 1].
    for i in k..(n - k) {
        averages[i] = sum / size as i32;
        if i + k + 1 < n {
            sum -= nums[i - k];
            sum += nums[i + k + 1];
        }
        
    }
    averages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let k = 3;
        let expected = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
        assert_eq!(get_averages(nums, k), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![100000];
        let k = 0;
        let expected = vec![100000];
        assert_eq!(get_averages(nums, k), expected);
    }

    #[test]
    fn test_3() {
        let nums = vec![8];
        let k = 100000;
        let expected = vec![-1];
        assert_eq!(get_averages(nums, k), expected);
    }
}
