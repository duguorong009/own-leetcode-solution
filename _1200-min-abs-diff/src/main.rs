fn main() {
    println!("Hello, world!");
}

fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut arr = arr;
    let mut min_abs_diff = i32::MAX;
    arr.sort_unstable();
    for i in 0..arr.len() - 1 {
        min_abs_diff = min_abs_diff.min(arr[i + 1] - arr[i]);
    }
    for i in 0..arr.len() - 1 {
        if (arr[i + 1] - arr[i]) == min_abs_diff {
            res.push(vec![arr[i], arr[i + 1]]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![4, 2, 1, 3];
        let expected = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(minimum_abs_difference(arr), expected);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 3, 6, 10, 15];
        let expected = vec![vec![1, 3]];
        assert_eq!(minimum_abs_difference(arr), expected);
    }

    #[test]
    fn test_3() {
        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let expected = vec![vec![-14, -10], vec![19, 23], vec![23, 27]];
        assert_eq!(minimum_abs_difference(arr), expected);
    }
}
