fn main() {
    let nums: Vec<i32> = vec![4, 3, 2, 3, 5, 2, 1];
    let k = 4;
    assert_eq!(can_partition_k_subsets(nums, k), true);
}

fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    // let nums: Vec<i32> = nums;
    // let sum: i32 = nums.iter().sum();
    // let target = sum / k;

    // if nums.iter().any(|elem| elem.gt(&target)) {
    //     return false;
    // }
    // true

    let n = nums.len();
    let sum: i32 = nums.iter().sum();
    if sum % k != 0 {
        return false;
    }
    let mut visited: Vec<bool> = vec![false; n];

    true // fake
}

#[cfg(test)]
mod tests {
    use crate::can_partition_k_subsets;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![4, 3, 2, 3, 5, 2, 1];
        let k = 4;
        assert_eq!(can_partition_k_subsets(nums, k), true);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let k = 3;
        assert_eq!(can_partition_k_subsets(nums, k), false);
    }
}
