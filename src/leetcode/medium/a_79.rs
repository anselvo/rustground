pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let word_vec = word.chars().collect();
    (0..board.len()).any(|i| {
        (0..board[i].len()).any(|j| {
            dfs(&mut board, &word_vec, i, j, 0)
        })
    })
}

fn dfs(board: &mut Vec<Vec<char>>, word: &Vec<char>, x: usize, y: usize, i: usize) -> bool {
    if board[x][y] != word[i] { return false; }
    if i+1 == word.len() { return true; }
    let tmp = board[x][y];
    board[x][y] = '/';
    let ans = (x < board.len()-1 && dfs(board, word, x+1, y, i+1))
        || (x > 0 && dfs(board, word, x-1, y, i+1))
        || (y < board[x].len()-1 && dfs(board, word, x, y+1, i+1))
        || (y > 0 && dfs(board, word, x, y-1, i+1));
    board[x][y] = tmp;
    ans
}

#[test]
fn test() {
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCCED".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SEE".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCB".to_string()), false);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCESCFSADEE".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ASADFBCCEESE".to_string()), true);
    assert_eq!(exist(vec![vec!['A']], "A".to_string()), true);
    assert_eq!(exist(vec![vec!['a']], "A".to_string()), false);
    assert_eq!(exist(vec![vec!['a']], "a".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SECB".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SEC".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SECBA".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "BAS".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ESECCE".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ESECCEDFBASA".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "EEDASFCSECBA".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "DAS".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ESCCBA".to_string()), true);
    assert_eq!(exist(vec![vec!['a', 'b']], "ba".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ESEECCBFDASA".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "DECFSA".to_string()), true);
    assert_eq!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "CEESECBFD".to_string()), true);
    assert_eq!(exist(vec![vec!['E','D','C'],vec!['F','A','B'],vec!['G','H','I']], "ABCDEFGHI".to_string()), true);
}
