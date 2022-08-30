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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            if r.borrow().val < val {
                Some(Rc::new(RefCell::new(TreeNode{ val, left: Some(r), right: None })))
            } else {
                let right = r.borrow_mut().right.take();
                r.borrow_mut().right = Self::insert_into_max_tree(right, val);
                Some(r)
            }
        } else {
            Some(Rc::new(RefCell::new(TreeNode{ val, left: None, right: None })))
        }
    }
}

struct Solution;

fn main() {}