// #17

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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(n1), Some(n2)) => {
                let (n1, n2) = (n1.borrow(), n2.borrow());
                let mut root = TreeNode::new(n1.val + n2.val);
                root.left = Self::merge_trees(n1.left.clone(), n2.left.clone());
                root.right = Self::merge_trees(n1.right.clone(), n2.right.clone());
                return Some(Rc::new(RefCell::new(root)));
            }
            (None, Some(n)) | (Some(n), None) => return Some(n),
            (None, None) => return None,
        }
    }
    #[allow(dead_code)]
    pub fn merge_trees_01(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => return None,
            (Some(n), None) | (None, Some(n)) => return Some(n),
            (Some(left), Some(right)) => {
                let bor_left = left.borrow();
                let bor_right = right.borrow();
                let new_val = bor_left.val + bor_right.val;

                let mut new_tree = TreeNode::new(new_val);
                new_tree.left = Self::merge_trees_01(bor_left.left.clone(), bor_right.left.clone());
                new_tree.right =
                    Self::merge_trees_01(bor_left.right.clone(), bor_right.right.clone());

                return Some(Rc::new(RefCell::new(new_tree)));
            }
        }
    }
}
