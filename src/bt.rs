use std::str::Chars;

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    #[inline]
    fn get_boxid(r: usize, c: usize) -> usize {
        let rv = (r as f32 / 3.0).floor() * 3.0;
        let cv = (c as f32 / 3.0).floor();
        (rv + cv) as usize
    }

    fn solve_backtrack(
        board: &mut Vec<Vec<char>>,
        boxes: &mut Vec<Vec<i8>>,
        rows: &mut Vec<Vec<i8>>,
        cols: &mut Vec<Vec<i8>>,
        r: usize,
        c: usize,
    ) -> bool {
        #[inline]
        fn is_valid(boxes: i8, rows: i8, cols: i8) -> bool {
            boxes + rows + cols == 0
        }

        if r == board.len() || c == board[0].len() {
            return true;
        }
        if board[r][c] == '.' {
            for num in 1..=9 {
                let num_char = char::from_digit(num, 10).unwrap();
                board[r][c] = num_char;

                let box_id = get_boxid(r, c);
                if is_valid(
                    boxes[box_id][num as usize],
                    rows[r][num as usize],
                    cols[c][num as usize],
                ) {
                    //, num as usize) {
                    boxes[box_id][num as usize] = 1;
                    rows[r][num as usize] = 1;
                    cols[c][num as usize] = 1;

                    if c == board[0].len() - 1 {
                        if solve_backtrack(board, boxes, rows, cols, r + 1, 0) {
                            return true;
                        }
                    } else {
                        if solve_backtrack(board, boxes, rows, cols, r, c + 1) {
                            return true;
                        }
                    }

                    boxes[box_id][num as usize] = 0;
                    rows[r][num as usize] = 0;
                    cols[c][num as usize] = 0;
                }
                board[r][c] = '.';
            }
        } else {
            if c == board[0].len() - 1 {
                if solve_backtrack(board, boxes, rows, cols, r + 1, 0) {
                    return true;
                }
            } else {
                if solve_backtrack(board, boxes, rows, cols, r, c + 1) {
                    return true;
                }
            }
        }
        false
    }

    let len = board.len();
    let (mut boxes, mut rows, mut cols) = (
        vec![vec![0i8; len + 1]; len],
        vec![vec![0i8; len + 1]; len],
        vec![vec![0i8; len + 1]; len],
    );
    for r in 0..len {
        for c in 0..len {
            if board[r][c] != '.' {
                let v = board[r][c].to_digit(10).unwrap() as usize;
                let boxid = get_boxid(r, c);
                boxes[boxid][v] = 1;
                rows[r][v] = 1;
                cols[c][v] = 1;
            }
        }
    }

    solve_backtrack(board, &mut boxes, &mut rows, &mut cols, 0, 0);
}

fn partition(s: String) -> Vec<Vec<String>> {
    fn is_palindrome(mut begin: usize, mut end: usize, s: &String) -> bool {
        //println!("   pal?: {}:{}:{:?}", begin, end, &s);
        let b = s.as_bytes();
        while begin < end {
            if b[begin] != b[end] {
                return false;
            }
            begin += 1;
            end -= 1;
        }
        true
    }
    fn do_partition(
        start_idx: usize,
        s: &String,
        ps: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        //println!("{}:{}:{:?}", start_idx, s, ps);
        if start_idx == s.len() {
            result.push(ps.to_vec());
        }
        for i in start_idx..s.len() {
            if is_palindrome(start_idx, i, s) {
                let part = s[start_idx..i + 1].to_string();
                ps.push(part);
                do_partition(i + 1, s, ps, result);
                ps.pop();
            }
        }
    }

    let mut result = vec![Vec::<String>::new(); 0];
    do_partition(0, &s, &mut Vec::<String>::new(), &mut result);
    result
}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn is_valid(queen_in_cols: &[usize]) -> bool {
        if queen_in_cols.len() == 1 {
            return true;
        }
        let r = queen_in_cols.len() - 1;
        let c = queen_in_cols[r];
        for i in 0..r {
            if queen_in_cols[i] == c || (r - i) as i32 == (queen_in_cols[i] as i32 - c as i32).abs()
            {
                return false;
            }
        }
        true
    }

    fn gen_solution(n: usize, queen_in_cols: &mut Vec<usize>) -> Vec<String> {
        let mut sol = Vec::<String>::new();
        for r in 0..queen_in_cols.len() {
            let mut curr_row = String::new();
            for c in 0..n {
                curr_row.push(if c == queen_in_cols[r] { 'Q' } else { '.' });
            }
            sol.push(curr_row);
        }
        sol
    }

    fn do_solve(
        n: usize,
        row: usize,
        queen_in_cols: &mut Vec<usize>,
        result: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            result.push(gen_solution(n, queen_in_cols))
        } else {
            for col in 0..n {
                queen_in_cols.push(col);
                if is_valid(queen_in_cols) {
                    do_solve(n, row + 1, queen_in_cols, result);
                }
                queen_in_cols.pop();
            }
        }
    }

    let mut result = vec![vec!["".to_string(); 0]; 0];
    do_solve(n as usize, 0, &mut vec![], &mut result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/sudoku-solver/

    #[test]
    fn solve_sudoku_test() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let expected = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        solve_sudoku(&mut board);
        assert_eq!(expected, board);
    }

    // https://leetcode.com/problems/palindrome-partitioning/

    #[test]
    fn palindrome_partitioning_test() {
        //assert_eq!(vec![vec!["a"]], partition("a".to_string()));
        //assert_eq!(vec![vec!["a","a","b"], vec!["aa","b"]], partition("aab".to_string()));
        assert_eq!(
            vec![vec!["a", "b", "a"], vec!["aba"]],
            partition("abadda".to_string())
        );
    }

    // https://leetcode.com/problems/n-queens/

    #[test]
    fn solve_n_queens_test() {
        //assert_eq!(vec![vec!["Q...", "..Q"]], solve_n_queens(3));
        //assert_eq!(vec![vec!["Q"]], solve_n_queens(1));
        assert_eq!(
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ],
            solve_n_queens(4)
        );
    }
}
