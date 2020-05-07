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
      let first_ch = needle.chars().next().unwrap();
      for (i, ch) in haystack.char_indices() {
         if ch != first_ch {
            continue;
         }
         if let Some(s) = haystack.get(i..i + step) {
            if s != needle {
               continue;
            }
            return i as i32;
         }
      }
      -1
   }
   #[test]
   fn str_str_test() {
      assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
      assert_eq!(str_str("aaaaaa".to_string(), "abb".to_string()), -1);
      assert_eq!(str_str("aaaaaa".to_string(), "".to_string()), 0);
      assert_eq!(str_str("mississippi".to_string(), "issi".to_string()), 1);
   }
}
