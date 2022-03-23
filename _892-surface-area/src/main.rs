fn main() {
    println!("Hello, world!");
}

fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] > 0 {
                res += 2 + 4 * grid[i][j];
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if i > 0 {
                res -= 2 * i32::min(grid[i][j], grid[i - 1][j]);
            }
            if j > 0 {
                res -= 2 * i32::min(grid[i][j], grid[i][j - 1]);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(surface_area(grid), 34);
    }
    
    #[test]
    fn test_2() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(surface_area(grid), 32);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]];
        assert_eq!(surface_area(grid), 46);
    }
}