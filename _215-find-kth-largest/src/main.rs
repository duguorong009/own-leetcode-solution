fn main() {
    println!("Hello, world!");
}

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // Sort the "nums"
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(&a));

    // Return the kth element
    nums[k as usize - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        assert_eq!(find_kth_largest(nums, k), 5);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        assert_eq!(find_kth_largest(nums, k), 4);
    }
}