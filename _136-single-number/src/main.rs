use std::collections::HashSet;
fn main() {
    let nums = vec![2, 2, 1];
    assert_eq!(single_number(nums), 1);
}

fn single_number(nums: Vec<i32>) -> i32 {
    let mut num_set: HashSet<i32> = HashSet::new();
    for num in nums {
        if num_set.contains(&num) {
            num_set.remove(&num);
        } else {
            num_set.insert(num);
        }
    }
    for res in num_set {
        return res;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::single_number;

    #[test]
    fn test_1() {
        let nums = vec![2, 2, 1];
        assert_eq!(single_number(nums), 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number(nums), 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        assert_eq!(single_number(nums), 1);
    }
}
