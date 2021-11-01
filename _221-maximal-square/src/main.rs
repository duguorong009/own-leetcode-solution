fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(maximal_square(matrix), 4);
}

fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max = 0;
    let n = matrix.len();
    if n == 0 {
        return 0;
    }
    let m = matrix[0].len();
    if m == 0 {
        return 0;
    }
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if matrix[i - 1][j - 1] == '1' {
                dp[i][j] = i32::min(dp[i - 1][j - 1], i32::min(dp[i - 1][j], dp[i][j - 1])) + 1;
                max = i32::max(max, dp[i][j]);
            }
        }
    }
    max * max
}
#[cfg(test)]
mod tests {
    use crate::maximal_square;

    #[test]
    fn test_1() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(maximal_square(matrix), 4);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec!['0', '1'], vec!['1', '0']];
        assert_eq!(maximal_square(matrix), 1);
    }

    #[test]
    fn test_3() {
        let matrix = vec![vec!['0']];
        assert_eq!(maximal_square(matrix), 0);
    }
}
