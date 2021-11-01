fn main() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    assert_eq!(
        four_sum(nums, target),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );
}

fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let n = nums.len();
    if n < 4 {
        return vec![];
    }
    let mut res: Vec<Vec<i32>> = vec![];
    nums.sort_unstable();
    println!("{:?}", nums);
    for i in 0..n - 3 {
        let a = nums[i];

        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut m = i + 1;
        // println!("{}", m);
        while m < n - 2 {
            let mut j = m + 1;
            let mut k = n - 1;
            // println!("{}, {}, {}, {}", i, m, j, k);
            while j < k {
                let b = nums[m];
                let c = nums[j];
                let d = nums[k];
                // println!("{}, {}, {}, {}", a, b, c, d);
                let sum = a + b + c + d;
                if sum == target {
                    let ans: Vec<i32> = vec![a, b, c, d];
                    if !res.contains(&ans) {
                        res.push(ans);
                    }
                    j += 1;
                    k -= 1;

                    while j < n - 3 && j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }

                    while k < n - 1 && k > j && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else {
                    if sum > target {
                        k -= 1;
                    } else {
                        j += 1;
                    }
                }
            }
            m += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::four_sum;

    #[test]
    fn test_1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        assert_eq!(
            four_sum(nums, target),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        assert_eq!(four_sum(nums, target), vec![vec![2, 2, 2, 2]]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, -2, -5, -4, -3, 3, 3, 5];
        let target = -11;
        assert_eq!(four_sum(nums, target), vec![vec![-5, -4, -3, 1]]);
    }
}
