fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", spiral_order(matrix));
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::spiral_order;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
