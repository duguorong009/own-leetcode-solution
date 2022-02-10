fn main() {
    println!("Hello, world!");
}

// // Brute-force solution.
// fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
//     let mut cnt = 0;

//     let n = nums.len();
//     for i in 0..n {
//         let mut sum = 0;
//         for j in i..n {
//             sum += nums[j];
//             if sum == k {
//                 cnt += 1;
//                 break;
//             }
//         }
//     }
//     cnt
// }

use std::collections::HashMap;

// Efficient solution
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut res = 0;
    hm.insert(0, 1);
    for x in nums {
        sum += x;
        res += hm.get(&(sum - k)).unwrap_or(&0);
        *hm.entry(sum).or_default() += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(subarray_sum(nums, k), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(subarray_sum(nums, k), 2);
    }
}