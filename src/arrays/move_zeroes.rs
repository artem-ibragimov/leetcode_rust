/**
 * Move Zeroes
 * https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/567/
 */
#[cfg(test)]
mod test {
   fn move_zeroes(nums: &mut Vec<i32>) {
      let mut zeroes: usize = 0;
      nums.retain(|n| match *n != 0 {
         true => true,
         false => {
            zeroes += 1;
            false
         }
      });
      nums.extend(vec![0; zeroes]);
   }
   // fn move_zeroes(nums: &mut Vec<i32>) {
   //    let zeroes: usize = nums
   //       .iter()
   //       .filter(|n| **n == 0)
   //       .collect::<Vec<&i32>>()
   //       .len();
   //    nums.retain(|n| *n != 0);
   //    nums.extend(vec![0; zeroes]);
   // }

   #[test]
   fn move_zeroes_test() {
      let mut v = vec![0, 1, 0, 3, 12];
      move_zeroes(&mut v);
      assert_eq!(v, vec![1, 3, 12, 0, 0]);
   }
}
