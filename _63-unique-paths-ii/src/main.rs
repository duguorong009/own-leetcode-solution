fn main() {
    let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(unique_paths_with_obstacles(obstacle_grid), 2);
}

fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    if m == 1 && n == 1 {
        return 1 - obstacle_grid[0][0];
    }
    let mut paths: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if i == 1 && j == 1 {
                paths[i][j] = 1 - obstacle_grid[i - 1][j - 1];
            } else {
                if obstacle_grid[i - 1][j - 1] == 0 {
                    paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
                } else {
                    paths[i][j] = 0;
                }
            }
        }
    }
    paths[m][n]
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_with_obstacles;

    #[test]
    fn test_1() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 2);
    }

    #[test]
    fn test_2() {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 1);
    }

    #[test]
    fn test_3() {
        let obstacle_grid = vec![vec![1]];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 0);
    }

    #[test]
    fn test_4() {
        let obstacle_grid = vec![vec![1, 0]];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 0);
    }
}
