// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>,
}

impl ListNode {
   #[inline]
   pub fn new(val: i32) -> Self {
      ListNode { next: None, val }
   }
}

pub fn make_list(indeces: Vec<i32>) -> Option<Box<ListNode>> {
   if let Some(last) = indeces.last() {
      let mut list = ListNode::new(*last);
      for val in indeces.into_iter().rev().skip(1) {
         list = ListNode { val, next: Some(Box::new(list)) }
      }
      return Some(Box::new(list));
   }
   None
}
