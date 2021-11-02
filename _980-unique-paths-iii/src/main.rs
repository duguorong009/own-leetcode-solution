fn main() {
    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
    assert_eq!(unique_paths_iii(grid), 2);
}

fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut res = 0;
    let n = grid.len();
    let m = grid[0].len();
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 {
                count += 1;
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                dfs(i, j, count, &mut res, &mut grid, n, m);
            }
        }
    }
    res as i32
}

fn dfs(
    i: usize,
    j: usize,
    left: usize,
    all: &mut usize,
    grid: &mut Vec<Vec<i32>>,
    n: usize,
    m: usize,
) {
    match grid[i][j] {
        2 => {
            if left == 0 {
                *all += 1;
            }
        }
        1 => {
            grid[i][j] = -1;
            for (r, c) in adj(i, j, n, m) {
                dfs(r, c, left, all, grid, n, m);
            }
            grid[i][j] = 1;
        }

        0 => {
            grid[i][j] = -1;
            for (r, c) in adj(i, j, n, m) {
                dfs(r, c, left - 1, all, grid, n, m);
            }
            grid[i][j] = 0;
        }

        _ => {}
    }
}

fn adj(i: usize, j: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if i > 0 {
        res.push((i - 1, j));
    }
    if j > 0 {
        res.push((i, j - 1));
    }
    if i + 1 < n {
        res.push((i + 1, j));
    }
    if j + 1 < m {
        res.push((i, j + 1));
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_iii;

    #[test]
    fn test_1() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
        assert_eq!(unique_paths_iii(grid), 2);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        assert_eq!(unique_paths_iii(grid), 4);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![0, 1], vec![2, 0]];
        assert_eq!(unique_paths_iii(grid), 0);
    }
}
