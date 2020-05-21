// https://www.codewars.com/kata/52742f58faf5485cae000b9a/train/rust

#[cfg(test)]
mod tests {

   fn format_duration(seconds: u64) -> String {
      if seconds == 0 { return "now".to_string(); }
      let date: [(u64, &str); 5] =
         [(31536000, " year"), (86400, " day"), (3600, " hour"), (60, " minute"), (1, " second")];
      let mut result: Vec<String> = vec![];
      frmt(seconds, &mut result, &date);
      let last = result.pop().unwrap();
      if result.is_empty() { return last; }
      format!("{} and {}", result.join(", "), last)
   }
   fn frmt(sec: u64, result: &mut Vec<String>, date: &[(u64, &str)]) {
      for (val, name) in date.iter() {
         let div = sec / val;
         if div == 0 { continue; }
         let mut label: String = format!("{}{}", div, name);
         if div > 1 { label.push('s'); }
         result.push(label);
         return frmt(sec % val, result, date);
      }
   }
   #[test]
   fn test_basic() {
      assert_eq!(format_duration(1), "1 second");
      assert_eq!(format_duration(62), "1 minute and 2 seconds");
      assert_eq!(format_duration(120), "2 minutes");
      assert_eq!(format_duration(3600), "1 hour");
      assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
   }
}
