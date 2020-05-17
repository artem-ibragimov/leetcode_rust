// https://leetcode.com/explore/interview/card/top-interview-questions-easy/93/linked-list/560/
#[cfg(test)]
mod test {
   use super::super::list::ListNode;
   fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      if head.is_none() {
         return None;
      }
      let mut stack: Vec<i32> = vec![];
      let mut pointer = &head;
      while let Some(node) = pointer {
         stack.push(node.val);
         pointer = &node.next;
      }
      let first = stack.remove(0);
      let mut list = Some(Box::new(ListNode::new(first)));
      for val in stack.into_iter() {
         list = Some(Box::new(ListNode { val, next: list }));
      }
      list
   }
   use super::super::list::make_list;

   #[test]
   fn reverse_list_test() {
      let before = make_list(vec![1, 2, 3, 4, 5]);
      let after = make_list(vec![5, 4, 3, 2, 1]);
      assert_eq!(reverse_list(before), after);
   }
}
