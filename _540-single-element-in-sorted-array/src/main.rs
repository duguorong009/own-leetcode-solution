fn main() {
    println!("Hello, world!");
}

fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let mut m = l + (r - l) / 2;
        if m % 2 == 1 {
            m -= 1;
        }
        if nums[m + 1] == nums[m] {
            l = m + 2;
        } else {
            r = m;
        }
    }
    nums[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(single_non_duplicate(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(single_non_duplicate(nums), 10);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 1, 2];
        assert_eq!(single_non_duplicate(nums), 2);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 2, 2, 3, 3];
        assert_eq!(single_non_duplicate(nums), 1);
    }
}
