use std::cmp::Ordering::*;
fn main() {
    println!("Hello, world!");
}

fn search(nums: Vec<i32>, target: i32) -> bool {
    let n = nums.len();
    if n == 0 {
        return false;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] == target {
            return true;
        }
        match nums[m].cmp(&nums[r]) {
            Equal => {
                r -= 1;
            }
            Less => {
                if nums[m] < target && nums[r] >= target {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            Greater => {
                if nums[m] > target && nums[l] <= target {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }
    }
    nums[l] == target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        assert!(search(nums, target));
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        assert!(!search(nums, target));
    }
}