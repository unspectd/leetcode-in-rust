pub struct Solution;

use crate::util::linked_list::ListNode;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut current = head.as_mut().unwrap();
        while let Some(next) = current.next.as_mut() {
            if current.val == next.val {
                current.next = next.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::linked_list::to_list;

    #[test]
    fn test() {
        assert_eq!(Solution::delete_duplicates(linked![1, 1, 2]), linked![1, 2]);
        assert_eq!(
            Solution::delete_duplicates(linked![1, 1, 2, 3, 3]),
            linked![1, 2, 3]
        );
        assert_eq!(Solution::delete_duplicates(linked![]), linked![]);
    }
}