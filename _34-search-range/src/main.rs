fn main() {
    println!("Hello, world!");
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    match nums.binary_search(&target) {
        Ok(i) => {
            let mut l = i;
            let mut r = i;
            while l > 0 && nums[l - 1] == target {
                l -= 1;
            }
            while r + 1 < n && nums[r + 1] == target {
                r += 1;
            }
            vec![l as i32, r as i32]
        }
        Err(_) => vec![-1, -1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![3, 4]);
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![];
        let target = 0;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }
}