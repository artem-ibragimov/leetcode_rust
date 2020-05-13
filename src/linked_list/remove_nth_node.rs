// Remove Nth Node From End of List
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/93/linked-list/603/
#[cfg(test)]
mod test {
   fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
      if n == 0 {
         return head;
      }
      let mut right_pointer = &head;
      let mut indeces: Vec<i32> = vec![right_pointer.as_ref().unwrap().val];
      for _ in 0..=n {
         match &right_pointer.as_ref() {
            Some(pointer) => right_pointer = &pointer.next,
            None => break,
         }
      }
      let mut left_pointer = &head;
      while let Some(node) = right_pointer.as_ref() {
         left_pointer = &left_pointer.as_ref().unwrap().next;
         indeces.push(left_pointer.as_ref().unwrap().val);
         right_pointer = &node.next;
      }
      // todo wrap it back
      left_pointer = &left_pointer.as_ref().unwrap().next.as_ref().unwrap().next;
      while let Some(node) = left_pointer.as_ref() {
         indeces.push(left_pointer.as_ref().unwrap().val);
         left_pointer = &node.next;
      }
      let mut list = ListNode::new(*indeces.last().unwrap());
      for val in indeces.into_iter().rev().skip(1) {
         list = ListNode {
            val,
            next: Some(Box::new(list)),
         }
      }
      Some(Box::new(list))
   }
   #[test]
   fn remove_nth_from_end_test() {
      let before = make_list(vec![1, 2, 3, 4, 5]);
      let after = make_list(vec![1, 2, 3, 5]);
      assert_eq!(remove_nth_from_end(before, 2), after);
      let before = make_list(vec![1]);
      let after = make_list(vec![]);
      assert_eq!(remove_nth_from_end(before, 1), after);
      let before = make_list(vec![1, 2]);
      let after = make_list(vec![2]);
      assert_eq!(remove_nth_from_end(before, 2), after);
   }
   fn make_list(indeces: Vec<i32>) -> Option<Box<ListNode>> {
      if let Some(last) = indeces.last() {
         let mut list = ListNode::new(*last);
         for val in indeces.into_iter().rev().skip(1) {
            list = ListNode {
               val,
               next: Some(Box::new(list)),
            }
         }
         return Some(Box::new(list));
      }
      None
   }
   // Definition for singly-linked list.
   #[derive(PartialEq, Eq, Clone, Debug)]
   pub struct ListNode {
      pub val: i32,
      pub next: Option<Box<ListNode>>,
   }

   impl ListNode {
      #[inline]
      fn new(val: i32) -> Self {
         ListNode { next: None, val }
      }
   }
}
