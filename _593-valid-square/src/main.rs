use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn valid_square(
    p1: Vec<i32>,
    p2: Vec<i32>, 
    p3: Vec<i32>, 
    p4: Vec<i32>,
) -> bool {
    let mut hs: HashSet<i32> = HashSet::new();
    let v: Vec<Vec<i32>> = vec![p1, p2, p3, p4];
    for i in 0..4 {
        for j in i + 1..4 {
            hs.insert(distance(v[i].clone(), v[j].clone()));
        }
    }
    hs.len() == 2 && !hs.contains(&0)
}

fn distance(a: Vec<i32>, b: Vec<i32>) -> i32 {
    (a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 1];
        assert!(valid_square(p1, p2, p3, p4));
    }

    #[test]
    fn test_2() {
        let p1 = vec![0, 0];
        let p2 = vec![1, 1];
        let p3 = vec![1, 0];
        let p4 = vec![0, 12];
        assert!(!valid_square(p1, p2, p3, p4));
    }

    #[test]
    fn test_3() {
        let p1 = vec![1, 0];
        let p2 = vec![-1, 0];
        let p3 = vec![0, 1];
        let p4 = vec![0, -1];
        assert!(valid_square(p1, p2, p3, p4));
    }
}