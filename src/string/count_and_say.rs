// Count and Say
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/886/
#[cfg(test)]
mod test {
    fn count_and_say(n: i32) -> String {
        count(n, String::from("1")).chars().rev().collect()
    }
    fn count(n: i32, s: String) -> String {
        if n == 1 {
            return s;
        }
        let mut s2 = String::new();
        let mut counter: u32 = 0;
        let last_index: usize = s.len() - 1;
        let mut prev: char = s.chars().next().unwrap();
        for (i, c) in s.chars().enumerate() {
            if prev == c {
                counter += 1;
                if i != last_index {
                    continue;
                }
            }
            s2.push(prev);
            s2.push(std::char::from_digit(counter, 10).unwrap());
            if i == last_index && prev != c {
                s2.push(c);
                s2.push('1');
            }
            counter = 1;
            prev = c;
        }
        count(n - 1, s2)
    }
    #[test]
    fn count_and_say_test() {
        assert_eq!(count_and_say(1), "1");
        assert_eq!(count_and_say(2), "11");
        assert_eq!(count_and_say(3), "21");
        assert_eq!(count_and_say(4), "1211");
        assert_eq!(count_and_say(5), "111221");
        assert_eq!(count_and_say(6), "312211");
        assert_eq!(count_and_say(7), "13112221");
        assert_eq!(count_and_say(8), "1113213211");
        assert_eq!(count_and_say(9), "31131211131221");
        assert_eq!(count_and_say(10), "13211311123113112211");
    }
}
