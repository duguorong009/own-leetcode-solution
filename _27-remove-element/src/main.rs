fn main() {
    println!("Hello, world!");
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let n = nums.len();
    let mut size = 0;
    for i in 0..n {
        if nums[i] != val {
            nums[size] = nums[i];
            size += 1;
        }
    }
    size as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(remove_element(&mut nums, val), 2);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(remove_element(&mut nums, val), 5);
    }
}