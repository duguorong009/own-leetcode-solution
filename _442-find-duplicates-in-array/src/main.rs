use std::collections::HashMap;

fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    assert_eq!(find_duplicates(nums), vec![2, 3]);
}

fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 1 {
        return vec![];
    }
    let mut num_hash_table: HashMap<i32, u32> = HashMap::new();
    let mut res: Vec<i32> = vec![];
    for elem in nums {
        if num_hash_table.contains_key(&elem) {
            res.push(elem);
            num_hash_table.remove(&elem);
        } else {
            num_hash_table.insert(elem, 1);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::find_duplicates;

    #[test]
    fn test_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_duplicates(nums), vec![2, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 2];
        assert_eq!(find_duplicates(nums), vec![1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        assert_eq!(find_duplicates(nums), vec![])
    }
}
