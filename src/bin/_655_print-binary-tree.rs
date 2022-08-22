/*
Given the root of a binary tree, construct a 0-indexed m x n string matrix res that represents a formatted layout of the tree. The formatted layout matrix should be constructed using the following rules:

The height of the tree is height and the number of rows m should be equal to height + 1.
The number of columns n should be equal to 2height+1 - 1.
Place the root node in the middle of the top row (more formally, at location res[0][(n-1)/2]).
For each node that has been placed in the matrix at position res[r][c], place its left child at res[r+1][c-2height-r-1] and its right child at res[r+1][c+2height-r-1].
Continue this process until all the nodes in the tree have been placed.
Any empty cells should contain the empty string "".
Return the constructed matrix res.

 

Example 1:


Input: root = [1,2]
Output: 
[["","1",""],
 ["2","",""]]
Example 2:


Input: root = [1,2,3,null,4]
Output: 
[["","","","1","","",""],
 ["","2","","","","3",""],
 ["","","4","","","",""]]
 

Constraints:

The number of nodes in the tree is in the range [1, 210].
-99 <= Node.val <= 99
The depth of the tree will be in the range [1, 10].

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/print-binary-tree
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        use std::collections::VecDeque;

        let mut res: Vec<Vec<String>> = vec![];
        let mut deque = VecDeque::new();
        deque.push_back(root);

        let mut max_depth: i32 = -1;
        let mut child_node_exist = true;
        while child_node_exist {
            max_depth += 1;
            res.push(vec![]);
            child_node_exist = false;
            for _ in 0..deque.len() {
                res[max_depth as usize].push("".to_owned());
                let node = deque.pop_front().unwrap();
                match node {
                    Some(n) => {
                        let n = n.as_ref().borrow();
                        res[max_depth as usize].push(n.val.to_string());
                        deque.push_back(n.left.clone());
                        deque.push_back(n.right.clone());
                        if n.left.is_some() || n.right.is_some() { child_node_exist = true; }
                    }
                    None => {
                        res[max_depth as usize].push("".to_owned());
                        deque.push_back(None);
                        deque.push_back(None);
                    }
                }
            }
            res[max_depth as usize].remove(0);
        }

        for (depth, vec) in res.iter_mut().enumerate() {
            let fill_cnt: i32 = 2_i32.pow(max_depth as u32 - depth as u32) - 1;
            if fill_cnt > 0 {
                let right_index = 2_i32.pow(depth as u32 + 1) - 1;
                for i in (0..right_index).rev().step_by(2) {
                    for _ in 0..fill_cnt {
                        vec.insert(i as usize + 1, "".to_string());
                    }
                    for _ in 0..fill_cnt {
                        vec.insert(i as usize, "".to_string());
                    }
                }
            }
        }

        res
    }
}

struct Solution;

fn main() {
    let root = Some(Rc::new(RefCell::new(
        TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(
                        TreeNode {
                            val: 4,
                            left: None,
                            right: None,
                        }
                    ))),
                }
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }
            ))),
        }
    )));

    let vec = Solution::print_tree(root);
    println!("{:#?}", vec);
}