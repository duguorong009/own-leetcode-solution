fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate(&mut nums, k);
    println!("{:?}", nums);
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    // let k = k as usize % nums.len();
    // nums[..].reverse();
    // nums[0..k].reverse();
    // nums[k..].reverse();

    nums.rotate_right(k as usize)
}

#[cfg(test)]
mod tests {
    use crate::rotate;

    #[test]
    fn test_1() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let mut nums: Vec<i32> = vec![-1, -100, 3, 99];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}
