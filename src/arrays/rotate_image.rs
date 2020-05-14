/// ## Rotate Image
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/770/

#[cfg(test)]
mod rotate_image {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let mut stack: Vec<i32> = vec![];
        for column_i in 0..len {
            for row_i in (0..len).rev() {
                stack.push(matrix[row_i][column_i]);
            }
        }
        for row_i in (0..len).rev() {
            for column_i in (0..len).rev() {
                matrix[row_i][column_i] = stack.pop().unwrap_or_default();
            }
        }
    }

    #[test]
    fn rotate_test() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
        let mut matrix =
            vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
        rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]]
        );
    }
}
