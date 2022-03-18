fn main() {
    println!("Hello, world!");
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut a = vec![];
    for row in matrix {
        for x in row {
            a.push(x);
        }
    }
    a.binary_search(&target).is_ok()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert!(search_matrix(matrix, target));
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert!(!search_matrix(matrix, target));
    }
}