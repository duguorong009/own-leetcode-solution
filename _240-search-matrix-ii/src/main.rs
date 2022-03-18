use std::cmp::Ordering::*;

fn main() {
    println!("Hello, world!");
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix.len();
    if n == 0 {
        return false;
    }
    let m = matrix[0].len();
    if m == 0 {
        return false;
    }
    let mut i = 0;
    let mut j = m - 1;
    loop {
        match matrix[i][j].cmp(&target) {
            Equal => {
                break true;
            }
            Greater => {
                if j > 0 {
                    j -= 1;
                } else {
                    break false;
                }
            }
            Less => {
                if i + 1 < n {
                    i += 1;
                } else {
                    break false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]];
        let target = 5;
        assert!(search_matrix(matrix, target));
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]];
        let target =20;
        assert!(!search_matrix(matrix, target));
    }
}