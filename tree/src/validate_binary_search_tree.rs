// #22

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        Self::valid(root, std::i64::MIN, std::i64::MAX)
    }

    fn valid(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node) = node {
            if node.borrow().val as i64 <= min || node.borrow().val as i64 >= max {
                return false;
            }

            return Self::valid(node.borrow().left.clone(), min, node.borrow().val as i64)
                && Self::valid(node.borrow().right.clone(), node.borrow().val as i64, max);
        }
        true
    }

    pub fn is_valid_bst_01(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }

        false
    }

    fn is_valid(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => return true,
            Some(root_node) => {
                let mut bor_node = root_node.borrow();

                if bor_node.val as i64 <= min || bor_node.val as i64 >= max {
                    return false;
                }

                return Self::is_valid(bor_node.left.clone(), min, bor_node.val as i64)
                    && Self::is_valid(bor_node.right.clone(), bor_node.val as i64, max);
            }
        }
    }
}
