use core::num;

fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    assert_eq!(find_min(nums), 1);
}

fn find_min(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut h = nums.len() - 1;
    while l < h {
        if nums[l] < nums[h] {
            return nums[l];
        }
        let m = l + (h - l) / 2;
        if nums[l] <= nums[m] {
            l = m + 1;
        } else {
            h = m;
        }
    }
    nums[l]
}

#[cfg(test)]
mod tests {
    use crate::find_min;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min(nums), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(find_min(nums), 11);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_5() {
        let nums = vec![3, 1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(find_min(nums), 1);
    }
}
