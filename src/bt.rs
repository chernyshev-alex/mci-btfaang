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
                if is_valid(boxes[box_id][num as usize], rows[r][num as usize], cols[c][num as usize])  { //, num as usize) {
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
        vec![vec![0i8; len+1]; len],
        vec![vec![0i8; len+1]; len],
        vec![vec![0i8; len+1]; len],
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

#[cfg(test)]
mod test {
    use super::*;

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
          vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']];

        solve_sudoku(&mut board);
        assert_eq!(expected, board);
    }
}
