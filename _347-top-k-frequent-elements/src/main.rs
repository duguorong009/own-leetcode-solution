fn main() {
    println!("Hello, world!");
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    // Sort the "nums"
    let mut nums = nums;
    nums.sort_unstable();
    
    // Get the frequency of every characters in "nums"
    let mut freq: Vec<(i32, i32)> = vec![];
    let mut cnt = 1;
    let mut tmp = nums[0];
    for i in 1..n {
        if tmp == nums[i] {
            cnt += 1;
        } else {
            freq.push((tmp, cnt));
            tmp = nums[i];
            cnt = 1;
        }
    }
    freq.push((tmp, cnt));

    // sort the "freq" by frequency
    freq.sort_by(|&a, &b| b.1.cmp(&a.1));
    println!("{:?}", freq);

    // slice the first k elements from the sorted "freq"
    freq.into_iter().take(k as usize).map(|v| v.0).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(top_k_frequent(nums, k), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(top_k_frequent(nums, k), vec![1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![3,0,1,0];
        let k = 1;
        assert_eq!(top_k_frequent(nums, k), vec![0]);
    }
}