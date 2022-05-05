// #2

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();
        let mut curr = head.as_mut();
        while let Some(curr_node) = curr {
            stack.push(curr_node.val);
            curr = curr_node.next.as_mut();
        }

        let mut new_head = Box::new(ListNode::new(-1));
        let mut pre = new_head.as_mut();

        while let Some(new_val) = stack.pop() {
            let new_node = Some(Box::new(ListNode::new(new_val)));
            pre.next = new_node;
            pre = pre.next.as_mut().unwrap();
        }

        new_head.next
    }

    #[allow(dead_code)]
    pub fn reverse_list_01(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();
        let mut curr = head;
        while let Some(curr_node) = curr {
            stack.push(curr_node.val);
            curr = curr_node.next.clone();
        }

        let mut return_list = Box::new(ListNode::new(-1));
        let mut pre = return_list.as_mut();

        while let Some(new_val) = stack.pop() {
            let new_node = Box::new(ListNode::new(new_val));
            pre.next = Some(new_node);
            pre = pre.next.as_mut().unwrap();
        }
        return_list.next
    }

    #[allow(dead_code)]
    pub fn reserse_list_02(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_list_01() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        let list = Solution::reverse_list_01(head);
        println!("{:?}", list);
    }

    #[test]
    fn test_reverse_list_02() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let list = Solution::reverse_list_01(head);
        println!("{:?}", list);
    }
}
