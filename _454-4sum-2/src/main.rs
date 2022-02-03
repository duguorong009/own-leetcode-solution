use std::{collections::HashMap};

fn main() {
    println!("Hello, world!");
}
fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, usize> = HashMap::new();
    for &i in &nums1 {
        for &j in &nums2 {
            *hm.entry(i + j).or_default() += 1;
        }
    }

    let mut res = 0;
    for &i in &nums3 {
        for &j in &nums4 {
            if let Some(v) = hm.get(&(-i - j)) {
                res += v;
            }
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        assert_eq!(four_sum_count(nums1, nums2, nums3, nums4), 2);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![0];
        let nums2 = vec![0];
        let nums3 = vec![0];
        let nums4 = vec![0];
        assert_eq!(four_sum_count(nums1, nums2, nums3, nums4), 1);
    }
}