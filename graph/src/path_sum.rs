/*
#19
Given the root of a binary tree and an integer targetSum,
return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.

A leaf is a node with no children.

Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
Output: true
Explanation: The root-to-leaf path with the target sum is shown

*/

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    let mut sum = 0;

    match root {
        None => {
            return false;
        }
        Some(node) => {
            let bor_node = node.borrow();
            let val = bor_node.val;
            let left = bor_node.left.clone();
            let right = bor_node.right.clone();

            if left.is_none() && right.is_none() {
                return target_sum == val;
            } else {
                return has_path_sum(bor_node.left.clone(), target_sum - val)
                    || has_path_sum(bor_node.right.clone(), target_sum - val);
            }
        }
    }
}

pub fn has_path_sum1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        None => false,
        Some(root_node) => {
            let bor_node = root_node.borrow();

            match (&bor_node.left, &bor_node.right) {
                (None, None) => bor_node.val == target_sum,
                _ => {
                    has_path_sum1(bor_node.left.clone(), target_sum - bor_node.val)
                        || has_path_sum1(bor_node.right.clone(), target_sum - bor_node.val)
                }
            }
        }
    }
}
