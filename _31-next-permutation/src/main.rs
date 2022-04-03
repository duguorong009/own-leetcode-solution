fn main() {
    println!("Hello, world!");
}

fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut i = n - 1;
    while i > 0 && nums[i - 1] >= nums[i] {
        i -= 1;
    }
    let mut j = i;
    let mut k = n - 1;
    while j < k {
        nums.swap(j, k);
        j += 1;
        k -= 1;
    }
    if i > 0 {
        k = i;
        i -= 1;
        while nums[k] <= nums[i] {
            k += 1;
        }
        nums.swap(i, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![3, 2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![1, 1, 5];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}