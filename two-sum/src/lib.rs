///You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
///  Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

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

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut arr_1: Vec<i32> = vec![];
    let mut arr_2: Vec<i32> = vec![];

    while l1.is_some() {
        match l1.as_mut() {
            None => break,
            Some(node) => {
                arr_1.push(node.val);
                l1 = node.next.clone();
            }
        }
    }

    while l2.is_some() {
        match l2.as_mut() {
            None => break,
            Some(node) => {
                arr_2.push(node.val);
                l2 = node.next.clone();
            }
        }
    }

    let mut length_1 = arr_1.len() as i32;
    let mut length_2 = arr_2.len() as i32;
    let mut diff: i32;

    if length_1 > length_2 {
        diff = length_1 - length_2;
        for i in 0..diff {
            arr_2.push(0);
            length_2 += diff;
        }
    }
    if length_2 > length_1 {
        diff = length_2 - length_1;
        for i in 0..diff {
            arr_1.push(0);
            length_1 += diff;
        }
    }

    for i in 0..length_1 {
        let mut arr1_num = arr_1[i as usize];
        let mut arr2_num = arr_2[i as usize];

        if arr1_num + arr2_num >= 10 {
            arr_1[i as usize] = arr1_num + arr2_num - 10;
            arr_2[i as usize + 1] += 1;
        } else {
            arr_1[i as usize] = arr1_num + arr2_num;
        }
    }

    let mut return_list = Box::new(ListNode::new(-1));
    let mut new_node: ListNode;

    for i in 0..length_1 {
        new_node = ListNode::new(arr_1[i as usize]);
        return_list.next = Some(Box::new(new_node.clone()));
        return_list = return_list.next.unwrap();
    }

    Some(return_list)
}

pub fn add_two_numbers_1(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;
    let mut carry = 0;
    let mut v1 = 0;
    let mut v2 = 0;
    while l1.is_some() || l2.is_some() || carry != 0 {
        v1 = 0;
        v2 = 0;
        if let Some(node) = l1 {
            v1 = node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            v2 = node.val;
            l2 = node.next;
        }
        current.next = Some(Box::new(ListNode::new((v1 + v2 + carry) % 10)));
        current = current.next.as_mut().unwrap();
        carry = (v1 + v2 + carry) / 10;
    }

    head.next
}

pub fn add_two_numbers_2(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1.clone();
    let mut l2 = l2.clone();
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;

    let mut carry = 0;
    let mut v1 = 0;
    let mut v2 = 0;
    while l1.is_some() || l2.is_some() || carry != 0 {
        v1 = 0;
        v2 = 0;

        if let Some(node) = l1 {
            v1 = node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            v2 = node.val;
            l2 = node.next;
        }

        current.next = Some(Box::new(ListNode::new((v1 + v2 + carry) % 10)));
        current = current.next.as_mut().unwrap();
        carry = (v1 + v2 + carry) / 10;
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_two_numbers() {
        let mut arr1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let mut arr2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let added_array = add_two_numbers_2(arr1, arr2);
        println!("{:?}", added_array);
        // assert_eq!(result, 4);
    }
}
