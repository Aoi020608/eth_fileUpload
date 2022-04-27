// Definition for singly-linked list.
//Given the head of a sorted linked list,
//delete all duplicates such that each element appears only once.
//Return the linked list sorted as well.
use std::collections::HashSet;

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

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr_opt = head.as_mut();

    while let Some(curr) = curr_opt {
        let mut next_opt = curr.next.take();

        while let Some(next) = next_opt.as_mut() {
            if next.val == curr.val {
                next_opt = next.next.take();
            } else {
                curr.next = next_opt;
                break;
            }
        }
        curr_opt = curr.next.as_mut();
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_duplicates() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));
        let sorted = delete_duplicates(head);
        println!("{:?}", sorted);
    }
}
