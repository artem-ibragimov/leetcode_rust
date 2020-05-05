/// Valid Palindrome
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/883/

#[cfg(test)]
mod test {
   fn is_palindrome(s: String) -> bool {
      let chars: String = s
         .chars()
         .filter(|c| c.is_ascii_alphanumeric())
         .map(|c| c.to_ascii_lowercase())
         .collect::<String>();
      let half_len = chars.len() / 2;
      chars
         .bytes()
         .take(half_len)
         .eq(chars.bytes().rev().take(half_len))
   }

   #[test]
   fn is_palindrome_test() {
      assert_eq!(
         is_palindrome("A man, a plan, a canal: Panama".to_string()),
         true
      );
      assert_eq!(is_palindrome("race a car".to_string()), false);
   }
}
