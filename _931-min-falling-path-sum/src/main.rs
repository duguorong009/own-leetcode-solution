fn main() {
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    assert_eq!(min_falling_path_sum(matrix), 13);
}

fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    if n == 1 {
        return matrix[0][0];
    }
    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..n {
        res.push(vec![]);
        for j in 0..n {
            if i == 0 {
                res[i].push(matrix[0][j]);
            } else {
                let mut min_sum: i32 = 0;
                if j == 0 {
                    min_sum = res[i - 1][j].min(res[i - 1][j + 1]) + matrix[i][j];
                } else if j == n - 1 {
                    min_sum = res[i - 1][j].min(res[i - 1][j - 1]) + matrix[i][j];
                } else {
                    min_sum =
                        res[i - 1][j].min(res[i - 1][j - 1]).min(res[i - 1][j + 1]) + matrix[i][j];
                }
                res[i].push(min_sum);
            }
        }
    }
    *(res[n - 1].iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::min_falling_path_sum;

    #[test]
    fn test_1() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(min_falling_path_sum(matrix), 13);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(min_falling_path_sum(matrix), -59);
    }

    #[test]
    fn test_3() {
        let matrix = vec![vec![-48]];
        assert_eq!(min_falling_path_sum(matrix), -48);
    }
}
