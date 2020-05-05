/// First Unique Character in a String
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/881/

#[cfg(test)]
mod uniq_char {
   fn first_uniq_char(s: String) -> i32 {
      const CHAR_CODE: usize = 97;
      const ALPHABET_SIZE: usize = 26;
      let mut arr = [0; ALPHABET_SIZE];
      for c in s.chars() {
         arr[c as usize - CHAR_CODE] += 1;
      }
      for (i, c) in s.chars().enumerate() {
         if arr[c as usize - CHAR_CODE] == 1 {
            return i as i32;
         }
      }
      -1
   }

   #[test]
   fn first_uniq_char_test() {
      assert_eq!(first_uniq_char(String::from("leetcode")), 0);
      assert_eq!(first_uniq_char(String::from("loveleetcode")), 2);
   }
}
