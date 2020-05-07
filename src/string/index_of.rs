// Implement strStr()
// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/885/
#[cfg(test)]
mod test {
   fn str_str(haystack: String, needle: String) -> i32 {
      if needle.is_empty() {
         return 0;
      }
      if needle.len() > haystack.len() {
         return -1;
      }
      let step: usize = needle.len() as usize;
      let first_ch = needle.chars().nth(0).unwrap();
      for (i, ch) in haystack.chars().step_by(step).enumerate() {
         if ch == first_ch {
         }
      }
      -1
   }
   #[test]
   fn str_str_test() {
      // assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
      // assert_eq!(str_str("aaaaaa".to_string(), "abb".to_string()), -1);
      // assert_eq!(str_str("aaaaaa".to_string(), "".to_string()), 0);
   }
}
