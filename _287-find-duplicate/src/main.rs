fn main() {
    println!("Hello, world!");
}

fn find_duplicate(nums: Vec<i32>) -> i32 {
    let n = nums.len() - 1;
    let mut res = 0;
    // Use the property of XOR ( 0 ^ n ^ n = 0 )
    for i in 1..=n {
        res ^= i as i32 ^ nums[i];
    }
    res ^ nums[0]
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
}
