use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let mut properties = properties;
    properties.sort_by(|a, b| 
        match b[0].cmp(&a[0]) {
            Ordering::Equal => a[1].cmp(&b[1]),
            other => other,
        }
    );
    
    let mut defense = properties[0][1];
    for i in 1..properties.len() {
        if properties[i][1] < defense {
            res += 1;
        } else {
            defense = properties[i][1];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let properties = vec![vec![5, 5], vec![6, 3], vec![3, 6]];
        assert_eq!(number_of_weak_characters(properties), 0);
    }

    #[test]
    fn test_2() {
        let properties = vec![vec![2, 2], vec![3, 3]];
        assert_eq!(number_of_weak_characters(properties), 1);
    }

    #[test]
    fn test_3() {
        let properties = vec![vec![1, 5], vec![10, 4], vec![4, 3]];
        assert_eq!(number_of_weak_characters(properties), 1);
    }
}