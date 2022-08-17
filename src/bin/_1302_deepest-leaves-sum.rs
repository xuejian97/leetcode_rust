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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // BFS
        use std::collections::VecDeque;

        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root.clone());

        loop {
            let mut children_cnt = 0;
            let mut layer_value_sum = 0;
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    let n = node.as_ref().unwrap().borrow();
                    layer_value_sum += n.val;
                    match &n.left {
                        Some(l) => {
                            deque.push_back(Some(l.clone()));
                            children_cnt += 1;
                        },
                        None => {}
                    }

                    match &n.right {
                        Some(r) => {
                            deque.push_back(Some(r.clone()));
                            children_cnt += 1;
                        },
                        None => {}
                    }
                }
            }
            if children_cnt == 0 {
                return layer_value_sum
            }
        }
    }

    pub fn deepest_leaves_sum_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // DFS
        let mut sum = 0;
        let mut max_depth = 0;

        fn walk(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, func: &mut impl FnMut(i32, i32)) {
            if let Some(node) = node {
                func(depth, node.borrow().val);
                walk(node.borrow().left.clone(), depth + 1, func);
                walk(node.borrow().right.clone(), depth + 1, func);
            }
        }

        walk(root, 0, &mut |d, val| {
            if d == max_depth {
                sum += val;
            } else if d > max_depth {
                max_depth = d;
                sum = val;
            }
        });

        sum
    }
}

struct Solution;

fn main() {}