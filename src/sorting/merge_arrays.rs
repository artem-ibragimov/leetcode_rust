// Merge Sorted Array
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/96/sorting-and-searching/587/
#[cfg(test)]
mod test {
   fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
      if n == 0 {
         return;
      }
      let mut nums: Vec<i32> = vec![];
      let mut i1: usize = 0;
      let mut i2: usize = 0;
      while i2 < n as usize || i1 < m as usize {
         if i2 < n as usize && (nums2[i2] <= nums1[i1] || i1 >= m as usize) {
            nums.push(nums2[i2]);
            i2 += 1;
            continue;
         }
         if i1 < m as usize {
            nums.push(nums1[i1]);
            i1 += 1;
         }
      }
      std::mem::replace(nums1, nums);
   }
   #[test]
   fn merge_test() {
      let mut nums1 = vec![1, 2, 3, 0, 0, 0];
      let mut nums2 = vec![2, 5, 6];
      merge(&mut nums1, 3, &mut nums2, 3);
      assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

      let mut nums1 = vec![4, 0, 0, 0, 0, 0];
      let mut nums2 = vec![1, 2, 3, 5, 6];
      merge(&mut nums1, 1, &mut nums2, 5);
      assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
      let mut nums1 = vec![1];
      let mut nums2 = vec![0];
      merge(&mut nums1, 1, &mut nums2, 0);
      assert_eq!(nums1, vec![1]);
      let mut nums1 = vec![2, 0];
      let mut nums2 = vec![1];
      merge(&mut nums1, 1, &mut nums2, 1);
      assert_eq!(nums1, vec![1, 2]);
      let mut nums1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
      let mut nums2 = vec![1, 2, 2];
      merge(&mut nums1, 6, &mut nums2, 3);
      assert_eq!(nums1, vec![-1,0,0,1,2,2,3,3,3]);
   }
}
