fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        // Store the original length of "arr".
        let n = arr.len();

        // Duplicate every "0" element in "arr".
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                arr.insert(i + 1, 0);
                i += 1;
            }
            i += 1;
        }

        // Trim the extended "arr" so that its length becomes original one
        let diff = arr.len() - n;
        for _ in 0..diff {
            arr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn test_2() {
        let mut arr = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }
}