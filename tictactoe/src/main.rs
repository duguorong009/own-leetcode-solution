fn main() {
    let moves: Vec<Vec<i32>> = vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]];
    println!("{}", tictactoe(moves));
}

fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let win_cases: Vec<Vec<Vec<i32>>> = vec![
        vec![vec![0, 0], vec![0, 1], vec![0, 2]],
        vec![vec![1, 0], vec![1, 1], vec![1, 2]],
        vec![vec![2, 0], vec![2, 1], vec![2, 2]],
        vec![vec![0, 0], vec![1, 0], vec![2, 0]],
        vec![vec![0, 1], vec![1, 1], vec![2, 1]],
        vec![vec![0, 2], vec![1, 2], vec![2, 2]],
        vec![vec![0, 0], vec![1, 1], vec![2, 2]],
        vec![vec![2, 0], vec![1, 1], vec![0, 2]],
    ];
    // let a_moves: Vec<Vec<i32>> = moves
    //     .into_iter()
    //     .enumerate()
    //     .filter(|(id, _)| *id % 2 == 0)
    //     .map(|(_, value)| value)
    //     .collect();
    // let b_moves: Vec<Vec<i32>> = moves
    //     .into_iter()
    //     .enumerate()
    //     .filter(|(id, _)| *id % 2 == 1)
    //     .map(|(_, value)| value)
    //     .collect();
    // println!("{:?}\n{:?}", a_moves, b_moves);

    "A".to_string()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::tictactoe;

    #[test]
    fn test_1() {
        let moves: Vec<Vec<i32>> = vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]];
        assert_eq!(tictactoe(moves), "A".to_string());
    }

    #[test]
    fn test_2() {
        let moves: Vec<Vec<i32>> = vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0],
        ];
        assert_eq!(tictactoe(moves), "B".to_string());
    }

    #[test]
    fn test_3() {
        let moves: Vec<Vec<i32>> = vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2],
        ];
        assert_eq!(tictactoe(moves), "Draw".to_string());
    }

    #[test]
    fn test_4() {
        let moves: Vec<Vec<i32>> = vec![vec![0, 0], vec![1, 1]];
        assert_eq!(tictactoe(moves), "Pending");
    }
}
