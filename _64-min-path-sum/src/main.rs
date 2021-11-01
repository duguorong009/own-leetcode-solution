fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(min_path_sum(grid), 7);
}

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    if m == 1 && n == 1 {
        return grid[0][0];
    }
    let mut min_sum_table: Vec<Vec<i32>> = vec![];
    for i in 0..m {
        min_sum_table.push(vec![0; n]);
        for j in 0..n {
            if i == 0 && j == 0 {
                min_sum_table[i][j] = grid[0][0]
            } else if i == 0 {
                min_sum_table[i][j] = min_sum_table[i][j - 1] + grid[i][j];
            } else if j == 0 {
                min_sum_table[i][j] = min_sum_table[i - 1][j] + grid[i][j];
            } else {
                min_sum_table[i][j] =
                    min_sum_table[i][j - 1].min(min_sum_table[i - 1][j]) + grid[i][j];
            }
        }
    }
    min_sum_table[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::min_path_sum;

    #[test]
    fn test_1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(grid), 7);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(min_path_sum(grid), 12);
    }
}
