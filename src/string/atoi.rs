/// String to Integer (atoi)
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/884/

#[cfg(test)]
mod test {
   fn my_atoi(s: String) -> i32 {
      let s_trimed = s.trim();
      let mut num: String = String::from("");
      for c in s_trimed.chars() {
         if !c.is_ascii_digit() && c != '-' && c != '+' {
            break;
         }
         num.push(c);
      }
      match num.parse() {
         Ok(n) => n,
         Err(ref e) => {
            let overflow = "123123123123123".parse::<i32>().err().unwrap();
            if *e == overflow {
               return std::i32::MAX;
            }
            let underflow = "-123123123123123".parse::<i32>().err().unwrap();
            if *e == underflow {
               return std::i32::MIN;
            }
            0
         }
      }
   }

   #[test]
   fn my_atoi_test() {
      assert_eq!(my_atoi("42".to_string()), 42);
      assert_eq!(my_atoi("   -42".to_string()), -42);
      // The first non-whitespace character is '-', which is the minus sign.
      // Then take as many numerical digits as possible, which gets 42.
      assert_eq!(my_atoi("   -42 with words ".to_string()), -42);
      // Conversion stops at digit '3' as the next character is not a numerical digit.
      assert_eq!(my_atoi("words and 987".to_string()), 0);
      //  The first non-whitespace character is 'w', which is not a numerical
      //  digit or a +/- sign. Therefore no valid conversion could be performed.
      assert_eq!(my_atoi("-91283472332".to_string()), std::i32::MIN);
      //  The number "-91283472332" is out of the range of a 32-bit signed integer.
      //  Thefore INT_MIN (−231) is returned.
      assert_eq!(my_atoi("2147483648".to_string()), std::i32::MAX);
      assert_eq!(my_atoi("+1".to_string()), 1);
      assert_eq!(my_atoi("-5-".to_string()), -5);
   }
}
