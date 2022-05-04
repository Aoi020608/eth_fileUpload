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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.clone();
        let mut l2 = l2.clone();
        let mut new_node = Box::new(ListNode::new(0));
        let mut current = &mut new_node;

        let mut carry = 0;
        let mut v1 = 0;
        let mut v2 = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            v1 = 0;
            v2 = 0;

            if let Some(l1_node) = l1 {
                v1 = l1_node.val;
                l1 = l1_node.next;
            }

            if let Some(l2_node) = l2 {
                v2 = l2_node.val;
                l2 = l2_node.next;
            }

            current.next = Some(Box::new(ListNode::new((v1 + v2 + carry) % 10)));
            current = current.next.as_mut().unwrap();
            carry = (v1 + v2 + carry) / 10;
        }

        new_node.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_duplicates() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let res = Solution::add_two_numbers(l1, l2);
        println!("{:?}", res);
    }
}
