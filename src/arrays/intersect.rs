/**
 *  Intersection of Two Arrays II
 * https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/674/
 */
#[cfg(test)]
mod tests {
    fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut long, short) = match nums1.len() >= nums2.len() {
            true => (nums1, nums2),
            false => (nums2, nums1),
        };
        long.sort();
        let mut intersection: Vec<i32> = vec![];
        for n in short {
            match long.binary_search(&n) {
                Ok(i) => {
                    intersection.push(*long.get(i).unwrap());
                    long.remove(i);
                }
                Err(_) => {}
            }
        }
        intersection
    }

    #[test]
    fn intersect_test() {
        assert_eq!(intersect(vec![1, 2], vec![2, 2]), [2]);
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), [2, 2]);
        assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), [4, 9]);
    }
}
