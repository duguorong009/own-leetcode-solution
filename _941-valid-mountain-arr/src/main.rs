fn main() {
    println!("Hello, world!");
}

fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let n = arr.len();
    if n < 3 {
        return false;
    }
    let mut is_decreasing = false;
    let mut peak_idx = 0;
    for i in 1..n {
        if arr[i] - arr[i - 1] == 0 {
            return false;
        }
        if !is_decreasing && arr[i] - arr[i - 1] < 0 {
            is_decreasing = true;
            peak_idx = i - 1;
        }
        if is_decreasing && (arr[i] - arr[i - 1] > 0) {
            return false;
        }
    }
    if peak_idx == 0 {
        return false;
    }
    if is_decreasing {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![2, 1];
        assert_eq!(valid_mountain_array(arr), false);
    }

    #[test]
    fn test_2() {
        let arr = vec![3, 5, 5];
        assert_eq!(valid_mountain_array(arr), false);
    }

    #[test]
    fn test_3() {
        let arr = vec![0, 3, 2, 1];
        assert_eq!(valid_mountain_array(arr), true);
    }


    #[test]
    fn test_4() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(valid_mountain_array(arr), false);
    }


    #[test]
    fn test_5() {
        let arr = vec![9, 8, 7, 6,5, 4, 3, 2, 1, 0];
        assert_eq!(valid_mountain_array(arr), false);
    }
}