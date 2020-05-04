/// ## Valid Sudoku 
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/769/
#[cfg(test)]
mod valid_sudoku {
   fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
      let mut uniq_rows_nums = [[false; 9]; 9];
      let mut uniq_columns_nums = [[false; 9]; 9];
      let mut uniq_grids_nums = [[[false; 9]; 3]; 3];

      for (row_i, char_row) in board.iter().enumerate() {
         let uniq_row_nums = &mut uniq_rows_nums[row_i];
         let uniq_grid_nums =  &mut uniq_grids_nums[row_i / 3];

         for (column_i, char_el) in char_row.iter().enumerate() {
            if let Some(num) = char_el.to_digit(10).map(|n| n as usize - 1) {
               let row_val = &mut uniq_row_nums[num];
               if *row_val {
                  return false;
               }
               *row_val = true;

               let column_val =&mut uniq_columns_nums[column_i][num];
               if *column_val {
                  return false;
               }
               *column_val = true;

               let grid_val = &mut uniq_grid_nums[column_i / 3][num];
               if *grid_val {
                  return false;
               }
               *grid_val = true;
            };
         }
      }
      true
   }
   #[test]
   fn is_valid_sudoku_test() {
      let board = vec![
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
      assert_eq!(is_valid_sudoku(board), true);
      let board = vec![
         vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
         vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
         vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
         vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
         vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
         vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
         vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
         vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
         vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
      ];
      assert_eq!(is_valid_sudoku(board), false);
      //  Explanation: Same as Example 1, except with the 5 in the top left corner being
      //  modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
      let board = vec![
         vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
         vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
         vec!['.', '9', '8', '6', '.', '.', '.', '6', '.'],
         vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
         vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
         vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
         vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
         vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
         vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
      ];
      assert_eq!(is_valid_sudoku(board), false);
   }
}
