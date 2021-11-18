fn main() {
    println!("Hello, world!");
}

fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let n = nums.len();
    for i in 0..n {
        let index: usize = (nums[i].abs() as usize) - 1;
        nums[index] = -nums[index].abs();
    }

    let mut res: Vec<i32> = vec![];
    for i in 1..=n {
        let index = i - 1;
        if nums[index] > 0 {
            res.push(i as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_disappeared_numbers(nums), vec![5, 6]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1];
        assert_eq!(find_disappeared_numbers(nums), vec![2]);
    }
}
