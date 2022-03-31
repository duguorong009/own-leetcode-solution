fn main() {
    println!("Hello, world!");
}

fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    let mut lo = *nums.iter().max().unwrap();
    let mut hi = nums.iter().sum();
    let n = nums.len();
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if split(&nums, mid, n) <= m {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn split(nums: &[i32], max: i32, n: usize) -> i32 {
    let mut sum = 0;
    let mut res = 1;
    for i in 0..n {
        if nums[i] + sum > max {
            sum = nums[i];
            res += 1;
        } else {
            sum += nums[i];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![7, 2, 5, 10, 8];
        let m = 2;
        assert_eq!(split_array(nums, m), 18);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let m = 2;
        assert_eq!(split_array(nums, m), 9);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 4, 4];
        let m = 3;
        assert_eq!(split_array(nums, m), 4);
    }
}