fn main() {
    println!("Hello, world!");
}

// // Brute-force solution
// fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
//     let mut res = 0;
//     let mut diff = i32::MAX;

//     let n = nums.len();
//     if n == 3 {
//         return nums.iter().sum();
//     }

//     for i in 0..n {
//         for j in i + 1..n {
//             for k in j + 1..n {
//                 let sum = nums[i] + nums[j] + nums[k];
//                 if (sum - target).abs() < diff {
//                     res = sum;
//                     diff = (sum - target).abs();
//                 }
//             }
//         }
//     }
//     res
// }

// Efficient solution using sort + right/left pointer. 
fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = 0;
    let mut diff = i32::MAX;

    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();

    for i in 0..n - 2 {
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum == target {
                return sum;
            }
            if (sum - target).abs() < diff {
                diff = (sum - target).abs();
                res = sum;
            }
            if nums[l] + nums[r] > target - nums[i] {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        assert_eq!(three_sum_closest(nums, target), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 0, 0];
        let target = 1;
        assert_eq!(three_sum_closest(nums, target), 0);
    }
}