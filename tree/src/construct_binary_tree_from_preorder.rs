/*
#23
Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary
tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
Output: [3,9,20,null,null,15,7]

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

// Depth First Search
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&preorder, &inorder);

    None
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
            left: helper(&preorder[1..(1 + pivot_idx)], &inorder[0..pivot_idx]),
            right: helper(&preorder[(1 + pivot_idx)..], &inorder[(pivot_idx + 1)..]),
        })));
    } else {
        return None;
    }
}

// Preorder first value is the root.
//
