fn main() {
    println!("Hello, world!");
}

fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            let c = grid[i][j];
            if dfs(
                i,
                j,
                0,
                std::usize::MAX,
                std::usize::MAX,
                &mut visited,
                &grid,
                c,
                n,
                m,
            ) {
                return true;
            }
        }
    }
    false
}

fn dfs(
    i: usize,
    j: usize,
    dist: usize,
    pi: usize,
    pj: usize,
    visited: &mut Vec<Vec<bool>>,
    grid: &[Vec<char>],
    c: char,
    n: usize,
    m: usize,
) -> bool {
    if dist >= 4 && visited[i][j] {
        return true;
    }
    if visited[i][j] {
        return false;
    }
    visited[i][j] = true;
    if i > 0 && grid[i - 1][j] == c && i - 1 != pi {
        if dfs(i - 1, j, dist + 1, i, j, visited, grid, c, n, m) {
            return true;
        }
    }
    if j > 0 && grid[i][j - 1] == c && j - 1 != pj {
        if dfs(i, j - 1, dist + 1, i, j, visited, grid, c, n, m) {
            return true;
        }
    }
    if i + 1 < n && grid[i + 1][j] == c && i + 1 != pi {
        if dfs(i + 1, j, dist + 1, i, j, visited, grid, c, n, m) {
            return true;
        }
    }
    if j + 1 < m && grid[i][j + 1] == c && j + 1 != pj {
        if dfs(i, j + 1, dist + 1, i, j, visited, grid, c, n, m) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a'],
        ];
        assert!(contains_cycle(grid));
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c'],
        ];
        assert!(contains_cycle(grid));
    }

    #[test]
    fn test_3() {
        let grid = vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a'],
        ];
        assert!(!contains_cycle(grid));
    }
}
