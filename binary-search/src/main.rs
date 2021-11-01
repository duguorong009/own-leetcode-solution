use std::cmp::Ordering::*;
fn main() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    println!("{}", search(nums, target));
}

// Use binary search & double pointer(left and right).
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;

    while l <= r {
        let mid = (l + r) / 2;
        match nums[mid].cmp(&target) {
            Equal => {
                return mid as i32;
            }
            Greater => {
                if mid < 1 {
                    break;
                }
                r = mid - 1;
            }
            Less => {
                if mid + 1 > n - 1 {
                    break;
                }
                l = mid + 1;
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
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(search(nums, target), -1);
    }
}
