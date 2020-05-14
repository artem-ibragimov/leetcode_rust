/// Reverse String
/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/879/

#[cfg(test)]
mod test {
    fn reverse_string(s: &mut Vec<char>) {
        if s.len() <= 1 {
            return;
        }
        let l = s.len() - 1;
        for i in 0..s.len() / 2 {
            s.swap(i, l - i);
        }
    }
    #[test]
    fn reverse_string_test() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, ['o', 'l', 'l', 'e', 'h']);
        let mut s = vec![];
        reverse_string(&mut s);
        assert_eq!(s, []);
        let mut s = vec!['h'];
        reverse_string(&mut s);
        assert_eq!(s, ['h']);
    }
}
