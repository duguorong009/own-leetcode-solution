use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut lookup_table: HashMap<i32, i32> = HashMap::new();
    for (id, num) in nums.iter().enumerate() {
        if let Some(val) = lookup_table.get(num) {
            res.push(*val);
            res.push(id as i32);
            break;
        } else {
            lookup_table.insert(target - *num, id as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 5];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
