fn main() {
    println!("Hello, world!");
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];

    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in 0..intervals.len() {
        if i == 0 {
            let temp = vec![intervals[0][0], intervals[0][1]];
            res.push(temp);
        } else {
            let id = res.len() - 1;
            let interval = &intervals[i];
            let temp = &res[id];
            if interval[0] <= temp[1] {
                res[id][1] = temp[1].max(interval[1]);
            } else {
                let interval = vec![intervals[i][0], intervals[i][1]];
                res.push(interval);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::merge;

    #[test]
    fn test_1() {
        let intervals: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected: Vec<Vec<i32>> = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge(intervals), expected);
    }

    #[test]
    fn test_2() {
        let intervals: Vec<Vec<i32>> = vec![vec![1, 4], vec![4, 5]];
        let expected: Vec<Vec<i32>> = vec![vec![1, 5]];
        assert_eq!(merge(intervals), expected);
    }
}
