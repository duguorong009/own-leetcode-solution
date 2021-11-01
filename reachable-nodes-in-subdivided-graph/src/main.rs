fn main() {
    let edges = vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]];
    let max_moves = 6_i32;
    let n = 3_i32;
    println!("{}", reachable_nodes(edges, max_moves, n));
}

fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let edges = vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]];
        let max_moves = 6_i32;
        let n = 3_i32;
        assert_eq!(reachable_nodes(edges, max_moves, n), 13);
    }

    #[test]
    fn test_2() {
        let edges = vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]];
        let max_moves = 10_i32;
        let n = 4_i32;
        assert_eq!(reachable_nodes(edges, max_moves, n), 23);
    }

    #[test]
    fn test_3() {
        let edges = vec![
            vec![1, 2, 4],
            vec![1, 4, 5],
            vec![1, 3, 1],
            vec![2, 3, 4],
            vec![3, 4, 5],
        ];
        let max_moves = 17_i32;
        let n = 5_i32;
        assert_eq!(reachable_nodes(edges, max_moves, n), 1);
    }
}
