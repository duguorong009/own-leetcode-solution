fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_string();
    assert_eq!(exist(board, word), true);
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut board = board;
    let m = board.len();
    let n = board[0].len();
    let word = word.chars().collect::<Vec<char>>();
    if m == 1 && n == 1 {
        if word.len() != 1 {
            return false;
        } else if word[0] != board[0][0] {
            return false;
        }
    }
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == word[0] && dfs(&mut board, i as i32, j as i32, 0, &word) == true {
                return true;
            }
        }
    }
    false
}

fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, count: usize, word: &Vec<char>) -> bool {
    if count == word.len() {
        return true;
    }

    if i < 0
        || i >= board.len() as i32
        || j < 0
        || j >= board[0].len() as i32
        || board[i as usize][j as usize] != word[count]
    {
        return false;
    }

    let temp: char = board[i as usize][j as usize];
    board[i as usize][j as usize] = ' ';
    let res = dfs(board, i + 1, j, count + 1, word)
        || dfs(board, i - 1, j, count + 1, word)
        || dfs(board, i, j + 1, count + 1, word)
        || dfs(board, i, j - 1, count + 1, word);
    board[i as usize][j as usize] = temp;
    return res;
}

#[cfg(test)]
mod tests {
    use crate::exist;

    #[test]
    fn test_1() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        assert_eq!(exist(board, word), true);
    }

    #[test]
    fn test_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        assert_eq!(exist(board, word), true);
    }

    #[test]
    fn test_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        assert_eq!(exist(board, word), false);
    }
}
