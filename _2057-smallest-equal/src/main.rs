fn main() {
    println!("Hello, world!");
}

fn smallest_equal(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        if i % 10 == nums[i] as usize {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![0, 1, 2];
        assert_eq!(smallest_equal(nums), 0);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 3, 2, 1];
        assert_eq!(smallest_equal(nums), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(smallest_equal(nums), -1);
    }
}