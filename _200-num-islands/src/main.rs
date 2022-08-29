fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let m = grid.len();
        let n = grid[0].len();

        let mut grid = grid;
        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if visited[i][j] == false && grid[i][j] == '1' {
                    count += 1;
                    Self::dfs(&mut grid, &mut visited, i, j, m, n);
                }
            }
        }

        count
    }

    fn dfs(
        grid: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) {
        if visited[i][j] == true || grid[i][j] == '0' {
            return;
        }
        visited[i][j] = true;

        if i >= 1 {
            Self::dfs(grid, visited, i - 1, j, m, n)
        }

        if i + 1 < m {
            Self::dfs(grid, visited, i + 1, j, m, n)
        }

        if j >= 1 {
            Self::dfs(grid, visited, i, j - 1, m, n)
        }

        if j + 1 < n {
            Self::dfs(grid, visited, i, j + 1, m, n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ])
        );
    }
}
