fn main() {
    println!("Hello, world!");
}

fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let k = k as usize;
    if m == 1 && n == 1 {
        return mat;
    }
    let mut res: Vec<Vec<i32>> = vec![vec![0; n + k]; m + k];
    for i in 0..m + k {
        for j in 0..n + k {
            // Task: Implement the function of DP & sum over here.
        }
    }
    res
}

fn elem_block_sum(res: &mut Vec<Vec<i32>>, i: usize, j: usize, k: usize) {}

#[cfg(test)]
mod tests {
    use crate::matrix_block_sum;

    #[test]
    fn test_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let res = vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]];
        assert_eq!(matrix_block_sum(mat, k), res);
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 2;
        let res = vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]];
        assert_eq!(matrix_block_sum(mat, k), res);
    }
}
