#[cfg(test)]
mod tests {

   fn single_number(nums: Vec<i32>) -> i32 {
      let mut xor: i32 = 0;
      for n in nums {
         xor = xor ^ n;
      }
      xor
   }

   #[test]
   fn single_number_test() {
      assert_eq!(single_number(vec![2, 2, 1]), 1);
      assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
   }
}
