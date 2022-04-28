// Given the head of a singly linked list, reverse the list, and return the reversed list.
// Definition for singly-linked list.
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]

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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = Vec::new();
    let mut curr = head;
    let mut return_list = ListNode::new(0);

    while let Some(curr_node) = curr {
        // println!("Current_node: {:?}", curr_node);         ListNode { val: 1, next: Some(ListNode { val: 2, next: None }) }
        stack.push(ListNode::new(curr_node.val));
        curr = curr_node.next.clone();
    }

    // println!("After while, {:?}", stack);

    // while return_list.next.is_some() {
    //     match stack.pop() {
    //         None => break,
    //         Some(new_node) => {
    //             return_list.next = Some(Box::new(new_node.clone()));
    //             return_list = new_node;
    //         }
    //     }
    // }

    // while !stack.is_empty() {
    //     println!("Return List: {:?} ", return_list);
    //     match stack.pop() {
    //         None => {}
    //         Some(node) => {
    //             return_list.next = ;
    //         }
    //     }
    // }

    return_list.next
}

pub fn reverse_list_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut return_list: Option<Box<ListNode>> = None;
    let mut stack = Vec::new();
    let mut curr = head;

    while let Some(curr_node) = curr {
        // println!("Current_node: {:?}", curr_node);         ListNode { val: 1, next: Some(ListNode { val: 2, next: None }) }
        stack.push(Some(Box::new(ListNode::new(curr_node.val))));
        curr = curr_node.next.clone();
    }

    let mut count: u8 = 0;
    return_list = stack[0].clone();

    return_list
}

pub fn reserse_list_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);
    while let Some(mut node) = curr {
        curr = node.next;

        node.next = prev;

        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_list() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        let list = reserse_list_2(head);
        println!("{:?}", list);
    }

    #[test]
    fn test_reverse_list_1() {
        let mut head = Some(Box::new(ListNode {
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

        let list = reserse_list_2(head);
        println!("{:?}", list);
    }
}
