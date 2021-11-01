fn main() {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(island_perimeter(grid), 16);
}

fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut res: i32 = 0;
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                res += calc_square_perim(&grid, i, j, m, n);
            }
        }
    }
    res
}

fn calc_square_perim(grid: &Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize) -> i32 {
    let mut perim: i32 = 0;
    if i == 0 {
        perim += 1;
    } else if grid[i - 1][j] == 0 {
        perim += 1;
    }
    if i == m - 1 {
        perim += 1;
    } else if grid[i + 1][j] == 0 {
        perim += 1;
    }
    if j == 0 {
        perim += 1;
    } else if grid[i][j - 1] == 0 {
        perim += 1;
    }
    if j == n - 1 {
        perim += 1;
    } else if grid[i][j + 1] == 0 {
        perim += 1;
    }
    perim
}

#[cfg(test)]
mod tests {
    use crate::island_perimeter;

    #[test]
    fn test_1() {
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(island_perimeter(grid), 16);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1]];
        assert_eq!(island_perimeter(grid), 4);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![1, 0]];
        assert_eq!(island_perimeter(grid), 4);
    }
}
