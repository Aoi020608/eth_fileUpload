// Given the head of a sorted linked list,
// delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list.
// Return the linked list sorted as well.

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

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut curr_opt = head.as_mut();
    let mut return_list: Box<ListNode>;

    if let Some(first_node) = curr_opt.as_ref() {
        return_list = Box::new(ListNode::new(first_node.val));

        while let Some(curr) = curr_opt {
            let mut next_opt = curr.next.take();

            while let Some(next) = next_opt.as_mut() {
                if curr.val != next.val {
                    return_list.next = next.next.clone();
                    next_opt = next.next.clone();
                } else {
                    curr.next = next_opt;
                    break;
                }
            }
            curr_opt = curr.next.as_mut();
        }

        // return_list.next = None;

        return Some(return_list);
    } else {
        return None;
    }
}

pub fn delete_duplicates_1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr_node = head.as_mut();

    while let Some(curr) = curr_node {
        let mut next_node = curr.next.take();

        while let Some(next) = next_node.as_mut() {
            if next.val == curr.val {
                next_node = next.next.take();
            } else {
                curr.next = next_node;
                break;
            }
        }
        curr_node = curr.next.as_mut();
    }

    head
}

pub fn delete_duplicates_2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut head = head;
    let mut res = ListNode::new(-1);
    res.next = head;
    let mut cur = &mut res;
    let mut remove = cur.next.as_mut().unwrap().val - 1;

    while cur.next.is_some() {
        let val = cur.next.as_mut().unwrap().val;
        let mut next_val = val - 1;
        if let Some(n) = &cur.next.as_mut().unwrap().next {
            next_val = n.val;
        }
        if val == next_val || val == remove {
            remove = val;
            let next = cur.next.as_mut().unwrap().clone();
            cur.next = next.next;
            continue;
        }
        cur = cur.next.as_mut().unwrap();
    }

    return res.next;
}

pub fn delete_duplicates_3(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut return_list = ListNode::new(-1);
    return_list.next = head;
    let mut cur = &mut return_list;
    let mut remove = cur.next.as_mut().unwrap().val - 1;

    while cur.next.is_some() {
        let curr_val = cur.next.as_mut().unwrap().val;
        let mut next_val = curr_val - 1;
        if let Some(n) = &cur.next.as_mut().unwrap().next {
            next_val = n.val;
        }
        if curr_val == next_val || curr_val == remove {
            remove = curr_val;
            let next = cur.next.as_mut().unwrap().clone();
            cur.next = next.next;
            continue;
        }
        cur = cur.next.as_mut().unwrap();
    }

    return return_list.next;
}

pub fn delete_duplicates_4(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut return_list = ListNode::new(-1);
    return_list.next = head;
    let mut cur = &mut return_list;
    let remove = cur.next.as_mut().unwrap().val - 1;

    while cur.next.is_some() {
        let val = cur.next.as_mut().unwrap().val;
        let mut next_val;
        if let Some(n) = &cur.next.as_mut().unwrap().next {
            next_val = n.val;
        }
        // if val == next_val || val == remove {

        // }
    }


    return return_list.next


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

        let sorted = delete_duplicates_3(head);
        println!("{:?}", sorted);
    }
}
