// Remove Nth Node From End of List
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/93/linked-list/603/
#[cfg(test)]
mod test {
   // fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
   //    if n == 0 {
   //       return head;
   //    }
   //    let mut tail = &head.unwrap().next;
   //    for i in 1..n {
   //       tail = &tail.unwrap().next;
   //       println!("tail {} {:?}", i, tail);
   //    }
   //    remove(&head, *tail)
   // }
   // fn remove(head: &Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
   //    if tail == None {
   //       return *head;
   //    }
   //    remove(&head.unwrap().next, tail.unwrap().next)
   // }
   #[test]
   fn remove_nth_from_end_test() {
      let before = Some(Box::new(make_list(vec![1, 2, 3, 4, 5])));
      let after = Some(Box::new(make_list(vec![1, 2, 3, 5])));
      assert_eq!(remove_nth_from_end(before, 2), after);
   }
   fn make_list(indeces: Vec<i32>) -> ListNode {
      let mut list = ListNode::new(*indeces.first().unwrap());
      for val in indeces.into_iter().skip(1) {
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
