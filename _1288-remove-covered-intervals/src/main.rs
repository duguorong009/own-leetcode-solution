fn main() {
    println!("Hello, world!");
}

fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {

    let mut intervals = intervals;
    let n = intervals.len();
    if n == 1 {
        return 1;
    }
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut removed: i32 = 0;
    let mut l = intervals[0][0];
    let mut r = intervals[0][1];
    for i in 1..n {
        if l == intervals[i][0] {
            removed += 1;
            r = r.max(intervals[i][1]);
        } else {
            if r >= intervals[i][1] {
                removed += 1;
            } else {
                l = intervals[i][0];
                r = intervals[i][1];
            }
        }
    }
    n as i32 - removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
        assert_eq!(remove_covered_intervals(intervals), 2);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        assert_eq!(remove_covered_intervals(intervals), 1);
    }

    #[test]
    fn test_3() {
        let intervals = vec![vec![3, 10], vec![4, 10], vec![5, 11]];
        assert_eq!(remove_covered_intervals(intervals), 2);
    }

    #[test]
    fn test_4() {
        let intervals = vec![vec![1,2], vec![1, 4], vec![3, 4]];
        assert_eq!(remove_covered_intervals(intervals), 1);
    }
}