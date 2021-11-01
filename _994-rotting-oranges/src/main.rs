fn main() {
    println!("Hello, world!");
}

fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let m = grid.len();
    let n = grid[0].len();
    let mut res: i32 = 0;
    // Implement the solution using queue and push & pop the rotten orange pos.
    let mut rottens: Vec<(usize, usize)> = vec![];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 2 {
                rottens.push((i, j));
            }
        }
    }
    while rottens.len() != 0 {
        let mut curr_rottens_len = rottens.len();
        let mut rotted = false;
        while curr_rottens_len != 0 {
            let (i, j) = rottens.remove(0);
            // Rotten the surrounding oranges
            if i > 0 && grid[i - 1][j] == 1 {
                grid[i - 1][j] = 2;
                rottens.push((i - 1, j));
                rotted = true;
            }
            if i < m - 1 && grid[i + 1][j] == 1 {
                grid[i + 1][j] = 2;
                rottens.push((i + 1, j));
                rotted = true;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                grid[i][j - 1] = 2;
                rottens.push((i, j - 1));
                rotted = true;
            }
            if j < n - 1 && grid[i][j + 1] == 1 {
                grid[i][j + 1] = 2;
                rottens.push((i, j + 1));
                rotted = true;
            }
            curr_rottens_len -= 1;
        }
        if rotted {
            res += 1;
        }
    }
    // Check if any fresh orange.
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                return -1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::oranges_rotting;

    #[test]
    fn test_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![0, 2]];
        assert_eq!(oranges_rotting(grid), 0);
    }
}
