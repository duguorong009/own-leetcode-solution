fn main() {
    println!("Hello, world!");
}

fn solve(board: &mut Vec<Vec<char>>) {
    let m = board.len();
    let n = board[0].len();
    if m < 2 || n < 2 {
        return;
    }
    for i in 0..m {
        if board[i][0] == 'O' {
            boundaryDFS(board, i, 0);
        }
        if board[i][n - 1] == 'O' {
            boundaryDFS(board, i, n - 1);
        }
    }

    for j in 0..n {
        if board[0][j] == 'O' {
            boundaryDFS(board, 0, j);
        }
        if board[m - 1][j] == 'O' {
            boundaryDFS(board, m - 1, j);
        }
    }

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 'O' {
                board[i][j] = 'X';
            } else if board[i][j] == '*' {
                board[i][j] = 'O';
            }
        }
    }
}

fn boundaryDFS(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
    if i > board.len() - 1 || i < 0 || j > board[0].len() - 1 || j < 0 {
        return;
    }

    if board[i][j] == 'O' {
        board[i][j] = '*';
    }

    if i > 0 && board[i - 1][j] == 'O' {
        boundaryDFS(board, i - 1, j);
    }

    if i < board.len() - 1 && board[i + 1][j] == 'O' {
        boundaryDFS(board, i + 1, j);
    }

    if j > 0 && board[i][j - 1] == 'O' {
        boundaryDFS(board, i, j - 1);
    }

    if j < board[0].len() - 1 && board[i][j + 1] == 'O' {
        boundaryDFS(board, i, j + 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        solve(&mut board);
        let res = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        assert_eq!(board, res);
    }
}
