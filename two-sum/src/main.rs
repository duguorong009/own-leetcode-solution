use std::collections::HashMap;
fn main() {
    println!("Hello world");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut table: HashMap<i32, usize> = HashMap::new();
    for (id, &num) in nums.iter().enumerate() {
        let target_minus_num = target - num;
        if !table.contains_key(&target_minus_num) {
            table.insert(num, id);
        } else {
            let id1 = *table.get(&target_minus_num).unwrap();
            res.push(id1 as i32);
            res.push(id as i32);
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
