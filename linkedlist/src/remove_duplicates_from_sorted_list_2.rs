use std::collections::HashMap;

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

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut p = head.as_mut();

        while let Some(cur) = p {
            *map.entry(cur.val).or_insert(0) += 1;
            p = cur.next.as_mut();
        }

        let mut new_head = Box::new(ListNode::new(0));
        new_head.next = head;

        let mut pre = new_head.as_mut();

        while let Some(cur) = pre.next.as_mut() {
            if let Some(v) = map.get(&cur.val) {
                if *v > 1 {
                    pre.next = cur.next.take();
                } else {
                    pre = pre.next.as_mut().unwrap();
                }
            } else {
                pre = pre.next.as_mut().unwrap();
            }
        }

        new_head.next
    }

    #[allow(dead_code)]
    pub fn delete_duplicates_01(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut map = HashMap::new();
        let mut cur_opt = head.as_mut();

        while let Some(node) = cur_opt {
            *map.entry(node.val).or_insert(0) += 1;
            cur_opt = node.next.as_mut();
        }

        let mut new_node = Box::new(ListNode::new(0));
        new_node.next = head;

        let mut pre = new_node.as_mut();

        while let Some(node) = pre.next.as_mut() {
            if let Some(val) = map.get(&node.val) {
                if *val > 1 {
                    pre.next = node.next.take();
                } else {
                    pre = pre.next.as_mut().unwrap();
                }
            } else {
                pre = pre.next.as_mut().unwrap();
            }
        }

        new_node.next
    }

    #[allow(dead_code)]
    pub fn delete_duplicates_02(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut to_remove = head.as_ref().unwrap().val - 1;
        let mut dummy = Some(Box::new(ListNode {
            next: head,
            val: to_remove,
        }));
        let mut node = &mut dummy.as_mut().unwrap().next;

        loop {
            match node {
                None => return dummy.unwrap().next,
                Some(n) if n.val == to_remove => *node = n.next.take(),
                Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => {
                    to_remove = n.val
                }
                Some(n) => {
                    node = &mut n.next;
                    if let Some(n) = node {
                        to_remove = n.val - 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_duplicates() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let sorted = Solution::delete_duplicates_02(head);
        println!("{:?}", sorted);
    }
}
