#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    fn rotate(nums: &mut Vec<i32>, k: i32) {
        let end: usize = (nums.len() as i32 - k % nums.len() as i32).try_into().unwrap_or_default();
        let offset = std::ops::Range::<usize> { start: 0, end };
        let stable: Vec<i32> = nums.drain(offset).collect();
        for el in stable {
            nums.push(el);
        }
    }

    #[test]
    fn rotate_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut v, 3);
        assert_eq!(v, [5, 6, 7, 1, 2, 3, 4]);
        v = vec![-1, -100, 3, 99];
        rotate(&mut v, 2);
        assert_eq!(v, [3, 99, -1, -100]);
        v = vec![-1];
        rotate(&mut v, 2);
        assert_eq!(v, [-1]);
        v = vec![1, 2];
        rotate(&mut v, 3);
        assert_eq!(v, [2, 1]);
    }
}
