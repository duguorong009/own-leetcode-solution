use std::vec;

fn main() {
    println!("Hello, world!");
}

fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res: Vec<Vec<i32>> = vec![];
    let n = nums.len();
    if n == 0 {
        return vec![];
    } else {
        let mut start = nums[0];
        let mut end = nums[0];
        for i in 1..n {
            if nums[i] - end == 1 {
                end = nums[i];
            } else {
                res.push(vec![start, end]);
                start = nums[i];
                end = nums[i];
            }
        }
        res.push(vec![start, end]);
    }

    res
        .into_iter()
        .map(|v| 
            if v[0] != v[1] {
                format!("{}->{}", v[0], v[1])
            } else { 
                format!("{}", v[0]) 
            })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        assert_eq!(summary_ranges(nums), vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        assert_eq!(summary_ranges(nums), vec!["0".to_string(), "2->4".to_string(), "6".to_string(), "8->9".to_string()]);
    }

    #[test]
    fn test_3() {
        let nums = vec![-1];
        assert_eq!(summary_ranges(nums), vec!["-1".to_string()]);
    }
}