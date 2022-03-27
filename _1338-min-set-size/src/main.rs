use std::collections::{HashMap};

fn main() {
    println!("Hello, world!");
}

fn min_set_size(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let target = n as i32 / 2;
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for i in 0..n {
        *(hm.entry(arr[i]).or_insert(0)) += 1;
    }

    let mut final_set: Vec<i32> = vec![];
    for val in hm.values() {
        final_set.push(*val);
    } 

    let total = final_set.len();
    final_set.sort_unstable();
    while final_set.iter().sum::<i32>() > target {
        final_set.pop();
    }

    (total - final_set.len()) as i32
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        assert_eq!(min_set_size(arr), 2);
    }

    #[test]
    fn test_2() {
        let arr = vec![7, 7, 7, 7, 7, 7];
        assert_eq!(min_set_size(arr), 1);
    }
}