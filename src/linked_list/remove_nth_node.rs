// Remove Nth Node From End of List
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/93/linked-list/603/
#[cfg(test)]
mod test {
   fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) {
      if n == 0 {
         // return head;
      }
      // TODO new linked list
      // let mut node_to_remove = &head;
      let mut last_node = &head;
      for i in 1..n {
         last_node = &(*last_node).as_ref().unwrap().next;
         println!(
            "fastforward last_node to {} ",
            (*last_node).as_ref().unwrap().val
         );
      }
      while let Some(node) = (*last_node).as_ref() {
         last_node = &node.next;
         // node_to_remove = &mut (*node_to_remove).as_ref().unwrap().next;
      }
      // *last_node
   }
   #[test]
   fn remove_nth_from_end_test() {
      let before = Some(Box::new(make_list(vec![1, 2, 3, 4, 5])));
      let after = Some(Box::new(make_list(vec![1, 2, 3, 5])));
      remove_nth_from_end(before, 2);
      // assert_eq!(remove_nth_from_end(before, 2), after);
   }
   fn make_list(indeces: Vec<i32>) -> ListNode {
      let mut list = ListNode::new(*indeces.last().unwrap());
      for val in indeces.into_iter().rev().skip(1) {
         list = ListNode {
            val,
            next: Some(Box::new(list)),
         }
      }
      list
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
