fn main() {
    println!("Hello, world!");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut last = nums[0];
    let mut size = 1;
    for i in 1..n {
        if nums[i] != last {
            last = nums[i];
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
        let mut nums: Vec<i32> = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
    }

    #[test]
    fn test_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }
}
