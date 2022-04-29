// #15
// Given the root of a binary tree, return its maximum depth.

// A binary tree's maximum depth is the number of nodes along the longest path from the
// root node down to the farthest leaf node.

// Definition for a binary tree node.
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

pub fn max_depth_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_depth = 0;
    let mut stack = vec![(root, 0)];
    while let Some((node, mut depth)) = stack.pop() {
        if let Some(node) = node {
            let node = node.borrow();
            depth += 1;
            max_depth = max_depth.max(depth);

            stack.push((node.left.clone(), depth));
            stack.push((node.right.clone(), depth));
        }
    }
    max_depth
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans = 0;
    let mut stack = vec![(root, 0)];

    while let Some((node, mut depth)) = stack.pop() {
        if let Some(node) = node {
            let node = node.borrow();
            depth += 1;
            ans = ans.max(depth);

            stack.push((node.left.clone(), depth));
            stack.push((node.right.clone(), depth));
        }
    }

    ans
}

pub fn max_depth_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut q: Vec<_> = vec![root.clone()];
    let mut depth = 1;
    while q.len() > 0 {
        let mut new_q: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        for node in q.iter() {
            if let Some(node) = node {
                let node = node.borrow();

                if node.left.is_some() {
                    new_q.push(node.left.clone());
                }

                if node.right.is_some() {
                    new_q.push(node.right.clone());
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

pub fn max_depth_3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut depth = 1;
    let mut q: Vec<_> = vec![root.clone()];

    while q.len() > 0 {
        let mut new_q: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        for opt in q.iter() {
            if let Some(node) = opt {
                let bor_node = node.borrow();

                if bor_node.left.is_some() {
                    new_q.push(bor_node.left.clone());
                }

                if bor_node.right.is_some() {
                    new_q.push(bor_node.right.clone());
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

// Input: root = [3,9,20,null,null,15,7]
// Output: 3

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_depth_1() {
        let result = Rc::new(RefCell::new(TreeNode::new(3)));
        assert_eq!(result, 4);
    }
}
