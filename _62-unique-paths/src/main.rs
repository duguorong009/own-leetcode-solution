fn main() {
    let m = 3;
    let n = 7;
    assert_eq!(unique_paths(m, n), 28);
}

fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut paths: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if i == 1 && j == 1 {
                paths[i][j] = 1;
            } else {
                paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
            }
        }
    }
    paths[m][n]
}

#[cfg(test)]
mod tests {
    use crate::unique_paths;

    #[test]
    fn test_1() {
        let m = 3;
        let n = 7;
        assert_eq!(unique_paths(m, n), 28);
    }

    #[test]
    fn test_2() {
        let m = 3;
        let n = 2;
        assert_eq!(unique_paths(m, n), 3);
    }

    #[test]
    fn test_3() {
        let m = 7;
        let n = 3;
        assert_eq!(unique_paths(m, n), 28);
    }

    #[test]
    fn test_4() {
        let m = 3;
        let n = 3;
        assert_eq!(unique_paths(m, n), 6);
    }

    #[test]
    fn test_5() {
        let m = 1;
        let n = 1;
        assert_eq!(unique_paths(m, n), 1);
    }
}
