fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    println!("{:?}", three_sum(nums));
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut res: Vec<Vec<i32>> = vec![];
    let n = nums.len();
    if n < 3 {
        return res;
    }
    nums.sort_unstable();
    for i in 0..n - 2 {
        let a = nums[i];
        if nums[i] > 0 {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let b = nums[j];
            let c = nums[k];
            let sum = a + b + c;
            if sum == 0 {
                res.push(vec![a, b, c]);
                j += 1;
                k -= 1;
                while j < k && nums[j] == nums[j - 1] {
                    j += 1;
                }
                while j < k && nums[k] == nums[k + 1] {
                    k -= 1;
                }
            } else {
                if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::three_sum;

    #[test]
    fn test_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test_2() {
        let nums = vec![];
        assert_eq!(three_sum(nums).len(), 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![0];
        assert_eq!(three_sum(nums).len(), 0);
    }

    #[test]
    fn test_4() {
        let nums = vec![0, 0, 0];
        assert_eq!(three_sum(nums), vec![vec![0, 0, 0]]);
    }
}
