/*
#21
Given the root of a binary tree, return the zigzag level order traversal of its nodes' values.
 (i.e., from left to right, then right to left for the next level and alternate between).
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

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(root_node) = root {
        queue.push_back(root_node);
    } else {
        return ans;
    }

    let mut switch = true;

    while !queue.is_empty() {
        if switch {
            ans.push(queue.iter().map(|node| node.borrow().val).collect());
        } else {
            let mut rev_arr = Vec::new();
            rev_arr.push(queue.iter().map(|node| &node.borrow().val).copied().collect::<i32>());
            rev_arr.reverse();
            ans.push(rev_arr);
        }

        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front() {
                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back(Rc::clone(&right.clone()));
                }
                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back(Rc::clone(&left.clone()));
                }
            }
        }
        switch = !switch;
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

pub fn zigzag_level_order_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    let mut zig = false;
    let mut queue = VecDeque::new();
    queue.push_front(root.unwrap());
    while !queue.is_empty() {
        let mut tmp = Vec::new();
        let len = queue.len();
        for _ in 0..len {
            let node = queue.pop_front().unwrap();
            tmp.push(node.borrow().val);
            if node.borrow().left.is_some() {
                queue.push_back(node.borrow().left.clone().unwrap());
            }
            if node.borrow().right.is_some() {
                queue.push_back(node.borrow().right.clone().unwrap());
            }
        }
        if zig {
            tmp.reverse();
        }
        zig = !zig;
        res.push(tmp);
    }

    res
}

// 3,9,20,null,null,15,7
// [3],[20,9],[15,7]
