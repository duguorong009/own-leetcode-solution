fn main() {
    println!("Hello, world!");
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut i = 0;
    let mut j = n - 1;
    while i <= j {
        let mid = i + (j - i) / 2;
        if target < nums[mid] {
            if mid < 1 {
                break;
            }
            j = mid - 1;
        } else if target > nums[mid] {
            if mid + 1 > n - 1 {
                break;
            }
            i = mid + 1;
        } else {
            return mid as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_3() {
        assert_eq!(search(vec![5], 5), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(search(vec![2, 5], 5), 1);
    }

    #[test]
    fn test_5() {
        assert_eq!(search(vec![5], -5), -1);
    }
}