/// https://leetcode.com/explore/featured/card/top-interview-questions-easy/127/strings/882/

#[cfg(test)]
mod test {
    fn is_anagram(s: String, t: String) -> bool {
        const ALPHABET_LEN: usize = 26;
        const CHAR_CODE: usize = 97;
        let mut chars = [0; ALPHABET_LEN];
        for byte in s.as_bytes() {
            chars[*byte as usize - CHAR_CODE] += 1;
        }
        for byte in t.as_bytes() {
            if chars[*byte as usize - CHAR_CODE] == 0 {
                return false;
            }
            chars[*byte as usize - CHAR_CODE] -= 1;
        }
        for i in 0..ALPHABET_LEN {
            if chars[i] != 0 {
                return false;
            }
        }
        true
    }

    #[test]
    fn is_anagram_test() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
