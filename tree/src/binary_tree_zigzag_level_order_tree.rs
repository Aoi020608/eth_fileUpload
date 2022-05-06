// #21
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(root_node) = root {
            queue.push_back(root_node);
        } else {
            return ans;
        }

        let mut switch = false;

        while queue.len() > 0 {
            let mut current_level: Vec<i32> = vec![];
            let mut next_level: Vec<Rc<RefCell<TreeNode>>> = vec![];

            for node in &queue {
                let x = node.borrow();
                current_level.push(x.val);
                let left = x.left.clone();
                let right = x.right.clone();
                if let Some(left) = left {
                    next_level.push(left);
                }
                if let Some(right) = right {
                    next_level.push(right);
                }
            }
            queue.clear();
            if switch {
                current_level.reverse();
            }
            switch = !switch;
            ans.push(current_level);
            for x in next_level {
                queue.push_back(x);
            }
        }

        ans
    }

    pub fn zigzag_level_order_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(root) = root {
            q.push_front(root);
        }
        let mut switch = false;
        while q.len() > 0 {
            let mut current_level: Vec<i32> = vec![];
            let mut next_level: Vec<Rc<RefCell<TreeNode>>> = vec![];
            for node in &q {
                let x = node.borrow();
                current_level.push(x.val);
                let left = x.left.clone();
                let right = x.right.clone();
                if let Some(left) = left {
                    next_level.push(left);
                }
                if let Some(right) = right {
                    next_level.push(right)
                }
            }
            q.clear();
            if switch {
                current_level.reverse();
            }
            switch = !switch;
            res.push(current_level);
            for x in next_level {
                q.push_back(x);
            }
        }
        res
    }

    // pub fn zigzag_level_order_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //     let mut res = Vec::new();
    //     if root.is_none() {
    //         return res;
    //     }
    //     let mut zig = false;
    //     let mut queue = VecDeque::new();
    //     queue.push_front(root.unwrap());
    //     while !queue.is_empty() {
    //         let mut tmp = Vec::new();
    //         let len = queue.len();
    //         for _ in 0..len {
    //             let node = queue.pop_front().unwrap();
    //             tmp.push(node.borrow().val);
    //             if node.borrow().left.is_some() {
    //                 queue.push_back(node.borrow().left.clone().unwrap());
    //             }
    //             if node.borrow().right.is_some() {
    //                 queue.push_back(node.borrow().right.clone().unwrap());
    //             }
    //         }
    //         if zig {
    //             tmp.reverse();
    //         }
    //         zig = !zig;
    //         res.push(tmp);
    //     }

    //     res
    // }
}
