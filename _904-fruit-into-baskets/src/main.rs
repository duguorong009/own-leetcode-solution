use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn total_fruit(fruits: Vec<i32>) -> i32 {
    let n = fruits.len();
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut start = 0;
    let mut res = 0;
    for end in 0..n {
        *last.entry(fruits[end]).or_default() = end;
        if last.len() == 3 {
            let (index, key) = last.iter().map(|(&k, &v)| (v, k)).min().unwrap();
            start = index + 1;
            last.remove(&key);
        }
        res = res.max(end - start + 1);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let fruits = vec![1, 2, 1];
        assert_eq!(total_fruit(fruits), 3);
    }

    #[test]
    fn test_2() {
        let fruits = vec![0, 1, 2, 2];
        assert_eq!(total_fruit(fruits), 3);
    }

    #[test]
    fn test_3() {
        let fruits = vec![1, 2, 3, 2, 2];
        assert_eq!(total_fruit(fruits), 4);
    }
}