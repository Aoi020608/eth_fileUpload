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

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut clone_head = head.as_mut();

        while let Some(curr_node) = clone_head {
            let mut next_node = curr_node.next.take();

            while let Some(next) = next_node.as_mut() {
                if curr_node.val == next.val {
                    next_node = next.next.take();
                } else {
                    curr_node.next = next_node;
                    break;
                }
            }

            clone_head = curr_node.next.as_mut();
        }

        head
    }

    #[allow(dead_code)]
    pub fn delete_duplicates_01(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root_opt = head.as_mut();

        while let Some(root_node) = root_opt {
            let mut next_opt = root_node.next.take();

            while let Some(next_node) = next_opt.as_mut() {
                if root_node.val == next_node.val {
                    next_opt = next_node.next.take();
                } else {
                    root_node.next = next_opt;
                    break;
                }
            }

            root_opt = root_node.next.as_mut()
        }
        head
    }

    pub fn delete_duplicates_02(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root_opt = head.as_mut();

        while let Some(root_node) = root_opt {
            let mut next_opt = root_node.next.take();

            while let Some(next_node) = next_opt.as_mut() {
                if root_node.val == next_node.val {
                    next_opt = next_node.next.take();
                } else {
                    root_node.next = next_opt;
                    break;
                }
            }

            root_opt = root_node.next.as_mut();
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_delete_duplicates() {
        let mut head = Some(Box::new(ListNode {
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

        let sorted = Solution::delete_duplicates(head);
        println!("{:?}", sorted);
    }
}
