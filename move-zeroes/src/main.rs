fn main() {
    let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    println!("{:?}", nums);
}

fn move_zeroes(nums: &mut Vec<i32>) {
    // // Rust-way
    // nums.sort();

    // Essence of the solution is to introduce the second pointer
    // "index" in addition to ordinary pointer like "i".
    let mut index = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[index] = nums[i];
            index += 1;
        }
    }

    for i in index..nums.len() {
        nums[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::move_zeroes;

    #[test]
    fn test_1() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        let expected: Vec<i32> = vec![1, 3, 12, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_2() {
        let mut nums: Vec<i32> = vec![0];
        let expected: Vec<i32> = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}
