use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");
}

fn minimum_deviation(nums: Vec<i32>) -> i32 {
    let mut min_deviation = i32::MAX;

    let mut heap = BinaryHeap::new();

    // Make every element to its possible maximum value.
    let mut nums = nums;
    let n = nums.len();
    for i in 0..n {
        if nums[i] % 2 == 1 {
            nums[i] *= 2;
        }
    }

    // Insert the values into binary heap(priority queue)
    let mut min = i32::MAX;
    for i in 0..n {
        heap.push(nums[i]);
        min = min.min(nums[i]);
    }
    min_deviation = min_deviation.min(heap.peek().unwrap() - min); 

    // While the "max" value is even, change the "max" value as much as possible.
    while heap.peek().unwrap() % 2 == 0 {
        let max = heap.pop().unwrap();
        heap.push(max / 2);
        min_deviation = min_deviation.min(heap.peek().unwrap() - min);
    }

    min_deviation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(minimum_deviation(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![4, 1, 5, 20, 3];
        assert_eq!(minimum_deviation(nums), 3);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![2, 10, 8];
        assert_eq!(minimum_deviation(nums), 3);
    }
}