fn main() {
    println!("Hello, world!");
}

fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
    let diff = (goal - nums.iter().sum::<i32>()).abs();
    (diff + limit - 1) / limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, -1, 1];
        let limit = 3;
        let goal = -4;
        assert_eq!(min_elements(nums, limit, goal), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, -10, 9, 1];
        let limit = 100;
        let goal = 0;
        assert_eq!(min_elements(nums, limit, goal), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![-1,0,1,1,1];
        let limit = 1;
        let goal = 771843707;
        assert_eq!(min_elements(nums, limit, goal), 771843705);
    }

    #[test]
    fn test_4() {
        let nums = vec![-4,0,-3,-3,0,0,-2,2,0,-2];
        let limit = 4;
        let goal = 940097744;
        assert_eq!(min_elements(nums, limit, goal), 235024439);
    }
}