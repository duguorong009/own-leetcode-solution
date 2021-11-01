fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'h'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];
    println!("{:?}", find_words(board, words));
}

fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut board = board;
    let mut res: Vec<String> = vec![];
    for word in words {
        let word_chars = word.chars().collect::<Vec<char>>();
        if check_exist(&mut board, &word_chars) {
            res.push(word);
        }
    }
    res
}

fn check_exist(board: &mut Vec<Vec<char>>, word: &Vec<char>) -> bool {
    let m = board.len();
    let n = board[0].len();
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == word[0] && dfs(board, i as i32, j as i32, 0, &word) {
                return true;
            }
        }
    }
    false
}

fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, count: usize, word: &Vec<char>) -> bool {
    let m = board.len();
    let n = board[0].len();
    if count == word.len() {
        return true;
    }
    if i < 0
        || i > m as i32 - 1
        || j < 0
        || j > n as i32 - 1
        || board[i as usize][j as usize] != word[count]
    {
        return false;
    }

    let temp = board[i as usize][j as usize];
    board[i as usize][j as usize] = ' ';
    let res = dfs(board, i - 1, j, count + 1, word)
        || dfs(board, i + 1, j, count + 1, word)
        || dfs(board, i, j - 1, count + 1, word)
        || dfs(board, i, j + 1, count + 1, word);
    board[i as usize][j as usize] = temp;
    res
}

#[cfg(test)]
mod tests {
    use crate::find_words;

    #[test]
    fn test_1() {
        let board = vec![
            vec!['o', 'a', 'a', 'h'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            "oath".to_string(),
            "pea".to_string(),
            "eat".to_string(),
            "rain".to_string(),
        ];
        assert_eq!(
            find_words(board, words),
            vec!["eat".to_string(), "oath".to_string()]
        );
    }

    #[test]
    fn test_2() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec!["abcb".to_string()];
        assert_eq!(find_words(board, words).len(), 0);
    }
}
