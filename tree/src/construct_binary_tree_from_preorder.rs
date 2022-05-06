// #23

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
    // Depth First Search
    #[allow(dead_code)]
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&preorder, &inorder)
    }

    fn helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = preorder.first() {
            let pivot_idx = inorder
                .iter()
                .enumerate()
                .find(|(_, v)| v == &root)
                .unwrap()
                .0;
            return Some(Rc::new(RefCell::new(TreeNode {
                val: *root,
                left: Self::helper(&preorder[1..(1 + pivot_idx)], &inorder[0..pivot_idx]),
                right: Self::helper(&preorder[(1 + pivot_idx)..], &inorder[(pivot_idx + 1)..]),
            })));
        } else {
            return None;
        }
    }

    pub fn build_tree_01(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper_01(&preorder, &inorder)
    }

    fn helper_01(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(first) = preorder.first() {
            let pivot_idx = inorder
                .iter()
                .enumerate()
                .find(|(_, val)| val == &first)
                .unwrap()
                .0;

            return Some(Rc::new(RefCell::new(TreeNode {
                val: *first,
                left: Self::helper_01(&preorder[1..=pivot_idx], &inorder[0..pivot_idx]),
                right: Self::helper_01(&preorder[(pivot_idx + 1)..], &inorder[(pivot_idx + 1)..]),
            })));
        } else {
            return None;
        }
    }
}
