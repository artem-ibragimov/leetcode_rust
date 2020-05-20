// Split and then add both sides of an array together.
// https://www.codewars.com/kata/5946a0a64a2c5b596500019a
#[cfg(test)]
mod test {

   fn split_and_add(arr: &[u32], n: usize) -> Vec<u32> {
      let len = arr.len();
      if len == 1 || n == 0 { return (*arr).to_vec(); }
      let (left, right) = arr.split_at(len / 2);
      let mut sum: Vec<u32> = right.iter().rev().zip(left.iter().rev()).map(|(&r, &l)| r + l).collect();
      if left.len() < right.len() { sum.push(right[0]); }
      sum.reverse();
      split_and_add(&sum, n - 1)
   }

   #[test]
   fn test_basics() {
      assert_eq!(split_and_add(&[1, 2, 3, 4, 5], 2), vec![5, 10]);
      assert_eq!(split_and_add(&[1, 2, 3, 4, 5], 3), vec![15]);
      assert_eq!(split_and_add(&[15], 3), vec![15]);
      assert_eq!(split_and_add(&[32, 45, 43, 23, 54, 23, 54, 34], 2), vec![183, 125]);
      assert_eq!(
         split_and_add(&[32, 45, 43, 23, 54, 23, 54, 34], 0),
         vec![32, 45, 43, 23, 54, 23, 54, 34]
      );
      assert_eq!(split_and_add(&[3, 234, 25, 345, 45, 34, 234, 235, 345], 3), vec![305, 1195]);
      assert_eq!(
         split_and_add(
            &[3, 234, 25, 345, 45, 34, 234, 235, 345, 34, 534, 45, 645, 645, 645, 4656, 45, 3],
            4
         ),
         vec![1040, 7712]
      );
      assert_eq!(
         split_and_add(&[23, 345, 345, 345, 34536, 567, 568, 6, 34536, 54, 7546, 456], 20),
         vec![79327]
      );
   }
}
