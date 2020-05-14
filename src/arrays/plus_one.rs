/**
 * Plus One
 * https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/559/
 */
#[cfg(test)]
mod test {
    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len();
        let mut ds: Vec<i32> = digits;
        for i in 1..=len {
            let v = ds.get(len - i).unwrap();
            if v + 1 < 10 {
                ds[len - i] = v + 1;
                return ds;
            }
            ds[len - i] = 0;
        }
        ds.insert(0, 1);
        ds
    }

    #[test]
    fn plus_one_test() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
        assert_eq!(plus_one(vec![1, 8, 9]), vec![1, 9, 0]);
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }
}
