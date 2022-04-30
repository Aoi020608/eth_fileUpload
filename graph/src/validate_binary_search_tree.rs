/*
Given the root of a binary tree, determine if it is a valid binary search tree (BST).

A valid BST is defined as follows:

The left subtree of a node contains only nodes with keys less than the node's key.
The right subtree of a node contains only nodes with keys greater than the node's key.
Both the left and right subtrees must also be binary search trees.

#22
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
use std::collections::VecDeque;
use std::rc::Rc;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut is_valid = true;

    if root.is_none() {
        return true;
    }

    valid(root, std::i64::MIN, std::i64::MAX)
}

fn valid(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    if let Some(node) = node {
        if node.borrow().val as i64 <= min || node.borrow().val as i64 >= max {
            return false;
        }

        return valid(node.borrow().left.clone(), min, node.borrow().val as i64)
            && valid(node.borrow().right.clone(), node.borrow().val as i64, max);
    }
    true
}
