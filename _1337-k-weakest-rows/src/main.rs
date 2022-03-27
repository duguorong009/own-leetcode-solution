fn main() {
    println!("Hello, world!");
}

fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    let m = mat.len();
    let mut indices: Vec<usize> = (0..m).into_iter().collect(); 

    let soldiers_cnt: Vec<usize> = mat
        .iter()
        .map(|v| v.iter().filter(|&n| *n == 1).count())
        .collect();
    indices.sort_by_key(|&i| soldiers_cnt[i]);
    
    for i in 0..k as usize {
        res.push(indices[i] as i32)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        let k = 3;
        assert_eq!(k_weakest_rows(mat, k), vec![2, 0, 3]);
    }

    #[test]
    fn test_2() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let k = 2;
        assert_eq!(k_weakest_rows(mat, k), vec![0, 2]);
    }
}
