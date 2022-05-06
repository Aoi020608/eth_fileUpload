// #20
use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret_vec: Vec<Vec<i32>> = vec![vec![]];
        match root {
            None => {
                return ret_vec;
            }
            Some(root_node) => {
                let bor_node = root_node.borrow();
                let mut layer_vec = vec![];
                ret_vec.push(vec![bor_node.val]);
                match (&bor_node.left, &bor_node.right) {
                    (Some(left_node), Some(right_node)) => {
                        layer_vec.push(left_node.borrow().val);
                        layer_vec.push(right_node.borrow().val);
                        Self::level_order(Some(left_node.clone()));
                        Self::level_order(Some(right_node.clone()));
                    }
                    (Some(left_node), None) => {
                        layer_vec.push(left_node.borrow().val);
                        Self::level_order(Some(left_node.clone()));
                    }
                    (None, Some(right_node)) => {
                        layer_vec.push(right_node.borrow().val);
                        Self::level_order(Some(right_node.clone()));
                    }
                    (None, None) => {
                        layer_vec.push(bor_node.val);
                    }
                }
                ret_vec.push(layer_vec);
            }
        }
        ret_vec
    }

    #[allow(dead_code)]
    pub fn level_order_01(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut ans = Vec::new();
        if let Some(node) = root {
            queue.push_back(node);
        } else {
            return ans;
        }
        while !queue.is_empty() {
            ans.push(queue.iter().map(|node| node.borrow().val).collect());
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(Rc::clone(&left));
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(Rc::clone(&right));
                    }
                }
            }
        }
        ans
    }

    #[allow(dead_code)]
    pub fn level_order_02(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = vec![];

        if let Some(node) = root {
            queue.push_back(node);
        } else {
            return ans;
        }

        while !queue.is_empty() {
            ans.push(queue.iter().map(|node| node.borrow().val).collect());

            for i in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    if let Some(left) = node.borrow().left {
                        queue.push_back(Rc::clone(&left));
                    }

                    if let Some(right) = node.borrow().right {
                        queue.push_back(Rc::clone(&right));
                    }
                }
            }
        }

        ans
    }
}
