fn main() {
    println!("Hello, world!");
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut l = 0 as usize;
    let mut r = n - 1;
    while l < r {
        let m = l + (r - l) / 2;
        if target < nums[m] {
            r = m;
        } else if target > nums[m] {
            l = m + 1;
        } else {
            return m as i32;
        }
    }
    if target <= nums[l] {
        l as i32
    } else {
        (l + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums, target), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums, target), 4);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }

    #[test]
    fn test_5() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }
}
