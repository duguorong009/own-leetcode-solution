fn main() {
    println!("Hello, world!");
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut prefix: Vec<i32> = vec![1; nums.len()];
    let mut suffix: Vec<i32> = vec![1; nums.len()];
    for i in 1..nums.len() {
        prefix[i] = prefix[i - 1] * nums[i - 1];
    }
    for i in (0..nums.len() - 1).rev() {
        suffix[i] = suffix[i + 1] * nums[i + 1];
    }
    for i in 0..nums.len() {
        res.push(prefix[i] * suffix[i]);
    }
    res
}

#[cfg(test)]
mod test {
    use crate::product_except_self;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }
}
