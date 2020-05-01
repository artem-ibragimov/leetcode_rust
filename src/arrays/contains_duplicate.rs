fn contains_dup(nums: Vec<i32>) -> bool {
   let mut sorted = nums;
   sorted.sort();
   for i in 1..sorted.len() {
      if sorted[i - 1] == sorted[i] {
         return true;
      }
   }
   false
}

#[cfg(test)]
mod tests {
   #[test]
   fn contains_duplicate_test() {
      assert_eq!(super::contains_dup(vec![1, 2, 3, 1]), true);
      assert_eq!(super::contains_dup(vec![1, 2, 3, 4]), false);
      assert_eq!(
         super::contains_dup(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
         true
      );
   }
}
