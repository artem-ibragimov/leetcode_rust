#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut prev: i32 = *nums.last().unwrap();
        let mut index: i32 = 0;
        nums.retain(|n| {
            let r = index == 0 || n != &prev;
            prev = *n;
            index += 1;
            r
        });
        nums.len().try_into().unwrap()
    }
    #[test]
    fn remove_duplicates_test() {
        assert_eq!(remove_duplicates(&mut vec![]), 0);
        let mut arr1 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut arr1), 5);
        assert_eq!(arr1, [0, 1, 2, 3, 4]);
        let mut arr2 = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut arr2), 2);
        assert_eq!(arr2, [1, 2]);
        let mut arr3 = vec![1, 1];
        assert_eq!(remove_duplicates(&mut arr3), 1);
        assert_eq!(arr3, [1]);
    }
}
