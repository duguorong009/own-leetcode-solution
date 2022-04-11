fn main() {
    println!("Hello, world!");
}

fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let size = m * n;

    let mut tmp: Vec<i32> = vec![];
    for row in grid.iter().take(m) {
        tmp.extend_from_slice(row);
    }

    let k = k as usize % size;
    tmp.rotate_right(k);

    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..size {
        if i % n == 0 {
            res.push(vec![tmp[i]]);
        } else {
            res[i / n].push(tmp[i]);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let k = 1;
        assert_eq!(shift_grid(grid, k), expected);
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let expected = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];
        let k = 4;
        assert_eq!(shift_grid(grid, k), expected);
    }
}
