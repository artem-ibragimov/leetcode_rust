/**
 * Two sum
 * https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/546/
 */
#[cfg(test)]
mod test {
   use std::collections::HashMap;
   fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
      let mut pairs = HashMap::new();
      for (i, num) in nums.iter().enumerate() {
         if let Some(pair) = pairs.get(&(target - num)) {
            return vec![*pair, i as i32];
         }
         pairs.insert(num, i as i32);
      }
      vec![]
   }
   #[test]
   fn two_sum_test() {
      assert_eq!(two_sum(vec![2, 7, 6, 15], 9), [0, 1]);
      assert_eq!(two_sum(vec![3, 2, 4], 6), [1, 2]);
   }
}
