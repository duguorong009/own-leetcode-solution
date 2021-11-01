use std::cmp::Ordering::*;
fn main() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(search_insert(nums, target), 2);
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut l: usize = 0;
    let mut r: usize = n - 1;
    while l <= r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&target) {
            Equal => {
                return mid as i32;
            }
            Greater => {
                if mid < 1 {
                    return 0;
                }
                r = mid - 1;
            }
            Less => {
                if mid + 1 > n - 1 {
                    return n as i32;
                }
                l = mid + 1;
            }
        }
    }
    l as i32
}

#[cfg(test)]
mod tests {
    use crate::search_insert;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums, target), 2);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums, target), 4);
    }

    #[test]
    fn test_4() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }

    #[test]
    fn test_5() {
        let nums: Vec<i32> = vec![1];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }
}
