// #19

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
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
                    return Self::has_path_sum(bor_node.left.clone(), target_sum - val)
                        || Self::has_path_sum(bor_node.right.clone(), target_sum - val);
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn has_path_sum1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(root_node) => {
                let bor_node = root_node.borrow();
                match (&bor_node.left, &bor_node.right) {
                    (None, None) => bor_node.val == target_sum,
                    _ => {
                        Self::has_path_sum1(bor_node.left.clone(), target_sum - bor_node.val)
                            || Self::has_path_sum1(
                                bor_node.right.clone(),
                                target_sum - bor_node.val,
                            )
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn has_path_sum02(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => return false,
            Some(node) => {
                let bor_node = node.borrow();
                match (&bor_node.left, &bor_node.right) {
                    (None, None) => target_sum == bor_node.val,
                    _ => {
                        Self::has_path_sum02(bor_node.left.clone(), target_sum - bor_node.val)
                            || Self::has_path_sum02(
                                bor_node.right.clone(),
                                target_sum - bor_node.val,
                            )
                    }
                }
            }
        }
    }
}
