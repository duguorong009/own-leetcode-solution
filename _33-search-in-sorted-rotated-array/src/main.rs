fn main() {
    println!("Hello, world!");
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        if nums[0] == target {
            return 0;
        } else {
            return -1;
        }
    }
    let mut l = 0 as usize;
    let mut h = nums.len() - 1;

    while l <= h {
        let m = (l + h) / 2;
        if target == nums[m] {
            return m as i32;
        } else if nums[l] <= nums[m] {
            if nums[l] <= target && target <= nums[m] {
                h = m - 1;
            } else {
                l = m + 1;
            }
        } else {
            if nums[m] <= target && target <= nums[h] {
                l = m + 1;
            } else {
                h = m - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn test_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test_4() {
        let nums = vec![0, 1, 2, 4, 5, 6, 7];
        let target = 7;
        assert_eq!(search(nums, target), 6);
    }

    #[test]
    fn test_5() {
        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 8;
        assert_eq!(search(nums, target), 4);
    }
}
