/// Reverse Integer
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/880/
#[cfg(test)]
mod test {
    fn reverse(x: i32) -> i32 {
        let mut s = x.to_string();
        while s.ends_with('0') {
            s.pop();
        }
        if s.starts_with('-') {
            let minus = s.remove(0);
            s.push(minus);
        }
        s.chars().rev().collect::<String>().parse::<i32>().unwrap_or_default()
    }
    #[test]
    fn reverse_test() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(103020), 20301);
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
    }
}
