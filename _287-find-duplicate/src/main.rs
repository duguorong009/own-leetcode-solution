fn main() {
    println!("Hello, world!");
}

fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = nums[0];
    let mut fast = nums[0];
    loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if slow == fast {
            break;
        }
    }
    slow = nums[0];
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }
    fast
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(nums), 2);
    }
    #[test]
    fn test_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(nums), 3);
    }
    #[test]
    fn test_3() {
        let nums = vec![1, 1];
        assert_eq!(find_duplicate(nums), 1);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 1, 2];
        assert_eq!(find_duplicate(nums), 1);
    }

    #[test]
    fn test_5() {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(find_duplicate(nums), 2);
    }
}
