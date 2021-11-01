fn main() {
    let mat = vec![vec![1, 2], vec![3, 4]];
    let r = 1;
    let c = 4;
    assert_eq!(matrix_reshape(mat, r, c), vec![vec![1, 2, 3, 4]]);
}

fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if mat.len() == 0 {
        return mat;
    }
    let m = mat.len();
    let n = mat[0].len();
    if (m * n) as i32 != r * c {
        return mat;
    }
    let mut mat_row: usize = 0;
    let mut mat_col: usize = 0;

    let mut result_mat: Vec<Vec<i32>> = vec![];
    for _i in 0..r {
        let mut row: Vec<i32> = vec![];
        for _j in 0..c {
            row.push(mat[mat_row][mat_col]);
            mat_col += 1;
            if mat_col == n {
                mat_row += 1;
                mat_col = 0;
            }
        }
        result_mat.push(row);
    }
    result_mat
}

#[cfg(test)]
mod tests {
    use crate::matrix_reshape;

    #[test]
    fn test_1() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let r = 1;
        let c = 4;
        assert_eq!(matrix_reshape(mat, r, c), vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let r = 2;
        let c = 4;
        assert_eq!(matrix_reshape(mat, r, c), vec![vec![1, 2], vec![3, 4]]);
    }
}
