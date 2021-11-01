fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    assert_eq!(jump(nums), 2);
}

fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut min_jumps: Vec<i32> = vec![0, 1];
    for i in 2..nums.len() {
        let mut temp: i32 = std::i32::MAX;
        for j in (0..i).rev() {
            if j + nums[j] as usize >= i {
                if temp > min_jumps[j] + 1 {
                    temp = min_jumps[j] + 1;
                }
            }
        }
        min_jumps.push(temp);
    }
    min_jumps[min_jumps.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::jump;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(jump(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 3, 1, 0, 4];
        assert_eq!(jump(nums), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![0];
        assert_eq!(jump(nums), 0);
    }
}
