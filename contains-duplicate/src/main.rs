use std::collections::HashMap;
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 5, 2, 1];
    println!("{}", contains_duplicate(nums));
}

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        if hash_map.contains_key(&num) {
            return true;
        } else {
            hash_map.insert(num, 1);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }
}
