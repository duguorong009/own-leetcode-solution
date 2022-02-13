fn main() {
    println!("Hello, world!");
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![]];
    let n = nums.len();
    partial_subsets(n, &nums, &mut res);
    res
}

fn partial_subsets(n: usize, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if n == 1 {
        res.push(vec![nums[0]]);
    } else {
        partial_subsets(n - 1, nums, res);
        for i in 0..res.len() {
            let mut tmp = res[i].to_owned();
            tmp.push(nums[n - 1]);
            res.push(tmp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        assert_eq!(subsets(nums), vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]]);
    }

    #[test]
    fn test_2() {
        let nums = vec![0];
        assert_eq!(subsets(nums), vec![vec![], vec![0]]);
    }
}