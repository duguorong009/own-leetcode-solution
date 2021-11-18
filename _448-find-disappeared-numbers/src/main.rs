use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
}

fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    if nums.len() == 0 {
        return res;
    }

    let mut distinct: HashSet<i32> = HashSet::new();
    for i in 0..nums.len() {
        distinct.insert(nums[i]);
    }
    for i in 1..nums.len() as i32 + 1 {
        if !distinct.contains(&i) {
            res.push(i);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_disappeared_numbers(nums), vec![5, 6]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1];
        assert_eq!(find_disappeared_numbers(nums), vec![2]);
    }
}
