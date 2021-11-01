fn main() {
    let nums = vec![2, 4, 1, 1, 7, 0, 5];
    assert_eq!(can_jump(nums), true);
}

fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut last = n - 1;
    for i in (0..n).rev() {
        if i + nums[i] as usize >= last {
            last = i;
        }
    }
    last == 0
}

#[cfg(test)]
mod tests {
    use crate::can_jump;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(can_jump(nums), true);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(can_jump(nums), false);
    }
}
