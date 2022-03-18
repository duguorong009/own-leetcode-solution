use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut hs: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for &x in &nums {
        let e = hs.entry(x).or_default();
        *e += 1;
    }
    for (x, u) in &hs {
        if let Some(v) = hs.get(&(x - 1)) {
            max = i32::max(u + v, max);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lhs() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        assert_eq!(find_lhs(nums), 5);

        let nums = vec![1, 2, 3, 4];
        assert_eq!(find_lhs(nums), 2);

        let nums = vec![1, 1, 1, 1];
        assert_eq!(find_lhs(nums), 0);
    }
}