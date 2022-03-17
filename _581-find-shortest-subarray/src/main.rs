fn main() {
    println!("Hello, world!");
}

fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut l = n - 1;
    let mut r = 0;

    for i in 0..n {
        max = i32::max(max, nums[i]);
        if max != nums[i] {
            r = i;
        }
    }

    for i in (0..n).rev() {
        min = i32::min(min, nums[i]);
        if min != nums[i] {
            l = i;
        }
    }
    if r <= l {
        0
    } else {
        (r - l + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        assert_eq!(find_unsorted_subarray(nums), 5);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(find_unsorted_subarray(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        assert_eq!(find_unsorted_subarray(nums), 0);
    }
}