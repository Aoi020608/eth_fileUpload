/*
#16
Given a binary tree, find its minimum depth.
The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.


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

// BFS

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut depth = 1;
    let mut q = vec![root.clone()];

    while q.len() > 0 {
        let mut new_q = vec![];

        for opt in q.iter() {
            if let Some(node) = opt {
                let bor_node = node.borrow();
                let mut left_none = false;
                let mut right_none = false;

                match &bor_node.left {
                    None => left_none = true,
                    Some(left) => {
                        new_q.push(bor_node.left.clone());
                    }
                }

                match &bor_node.right {
                    None => right_none = true,
                    Some(right) => {
                        new_q.push(bor_node.right.clone());
                    }
                }

                if left_none && right_none {
                    return depth;
                }
            }
        }

        if new_q.len() > 0 {
            depth += 1;
        }
        q = new_q;
    }
    depth
}

// DFS

//
