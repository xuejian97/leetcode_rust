/*
Given the root of a binary tree and two integers val and depth, add a row of nodes with value val at the given depth depth.

Note that the root node is at depth 1.

The adding rule is:

Given the integer depth, for each not null tree node cur at the depth depth - 1, create two tree nodes with value val as cur's left subtree root and right subtree root.
cur's original left subtree should be the left subtree of the new left subtree root.
cur's original right subtree should be the right subtree of the new right subtree root.
If depth == 1 that means there is no depth depth - 1 at all, then create a tree node with value val as the new root of the whole original tree, and the original tree is the new root's left subtree.


Example 1:


Input: root = [4,2,6,3,1,5], val = 1, depth = 2
Output: [4,1,1,2,null,null,6,3,1,5]
Example 2:


Input: root = [4,2,null,3,1], val = 1, depth = 3
Output: [4,2,null,1,1,3,null,null,1]


Constraints:

The number of nodes in the tree is in the range [1, 104].
The depth of the tree is in the range [1, 104].
-100 <= Node.val <= 100
-105 <= val <= 105
1 <= depth <= the depth of tree + 1

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/add-one-row-to-tree
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
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let new_root = TreeNode {
                val,
                left: root,
                right: None,
            };
            return Some(Rc::new(RefCell::new(new_root)));
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        for _ in 1..depth - 1 {
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    if node.is_none() {
                        continue;
                    }
                    queue.push_back(node.as_ref().unwrap().borrow().left.clone());
                    queue.push_back(node.as_ref().unwrap().borrow().right.clone());
                }
            }
        }

        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                if node.is_none() {
                    continue;
                }
                let new_node =
                    Some(Rc::new(RefCell::new(
                        TreeNode {
                            val,
                            left: node.as_ref().unwrap().borrow().left.clone(),
                            right: None,
                        }))
                    );

                node.as_ref().unwrap().borrow_mut().left = new_node;

                let new_node =
                    Some(Rc::new(RefCell::new(
                        TreeNode {
                            val,
                            left: None,
                            right: node.as_ref().unwrap().borrow().right.clone(),
                        }))
                    );

                node.as_ref().unwrap().borrow_mut().right = new_node;
            }
        }

        root
    }
}

struct Solution;

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    let root = Solution::add_one_row(root, 1, 2);

    let root_expected = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))),
    })));

    assert_eq!(root, root_expected);

}