// Remove Nth Node From End of List
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/93/linked-list/603/
#[cfg(test)]
mod test {
    use super::super::list::ListNode;
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == 0 {
            return head;
        }
        let mut pointer = &head;
        let mut indeces: Vec<i32> = vec![];
        while let Some(node) = pointer.as_ref() {
            indeces.push(pointer.as_ref().unwrap().val);
            pointer = &node.next;
        }
        indeces.remove(indeces.len() - n as usize);
        let last = match indeces.last() {
            Some(v) => v,
            None => return None,
        };
        let mut list = ListNode::new(*last);
        for val in indeces.into_iter().rev().skip(1) {
            list = ListNode { val, next: Some(Box::new(list)) }
        }
        Some(Box::new(list))
    }
    use super::super::list::make_list;
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
}
