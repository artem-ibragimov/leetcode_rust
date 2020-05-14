// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/887/
#[cfg(test)]
mod test {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        if let Some(init) = strs.first() {
            return strs.iter().fold(init.to_string(), |prefix, s| {
                prefix.chars().zip(s.chars()).take_while(|(p, c)| p == c).map(|(p, _)| p).collect()
            });
        }
        String::new()
    }
    #[test]
    fn longest_common_prefix_test() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
