/*
Given the root of a binary tree, return the length of the longest path, where each node in the path has the same value. This path may or may not pass through the root.

The length of the path between two nodes is represented by the number of edges between them.

 

Example 1:


Input: root = [5,4,5,1,1,null,5]
Output: 2
Explanation: The shown image shows that the longest path of the same value (i.e. 5).
Example 2:


Input: root = [1,4,5,4,4,null,5]
Output: 2
Explanation: The shown image shows that the longest path of the same value (i.e. 4).
 

Constraints:

The number of nodes in the tree is in the range [0, 104].
-1000 <= Node.val <= 1000
The depth of the tree will not exceed 1000.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-univalue-path
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
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_length = 0;
        fn walk(node: Option<Rc<RefCell<TreeNode>>>, func: &mut impl FnMut(i32)) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            match node {
                Some(n) => {
                    let (mut left_length, left_node) = walk(n.borrow().left.clone(), func);
                    let (mut right_length, right_node) = walk(n.borrow().right.clone(), func);
                    left_length = if left_node.is_some() && n.borrow().val == left_node.unwrap().borrow().val { left_length + 1 } else { 0 };
                    right_length = if right_node.is_some() && n.borrow().val == right_node.unwrap().borrow().val { right_length + 1 } else { 0 };
                    func(left_length + right_length);
                    (left_length.max(right_length), Some(n))
                }
                None => { (0, None) }
            }
        }

        walk(root, &mut |l| {
            if l > max_length {
                max_length = l;
            }
        });
        max_length
    }
}

struct Solution;

fn main() {
    // Some(Rc::new(RefCell::new(TreeNode{})))
    assert_eq!(
        Solution::longest_univalue_path(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
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
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
            })))
        ),
        4
    )

    // assert_eq!(
    //     Solution::longest_univalue_path(
    //         Some(Rc::new(RefCell::new(TreeNode {
    //             val: 1,
    //             left: Some(Rc::new(RefCell::new(TreeNode {
    //                 val: 4,
    //                 left: Some(Rc::new(RefCell::new(TreeNode {
    //                     val: 4,
    //                     left: None,
    //                     right: None,
    //                 }))),
    //                 right: Some(Rc::new(RefCell::new(TreeNode {
    //                     val: 4,
    //                     left: None,
    //                     right: None,
    //                 }))),
    //             }))),
    //             right: Some(Rc::new(RefCell::new(TreeNode {
    //                 val: 5,
    //                 left: None,
    //                 right: Some(Rc::new(RefCell::new(TreeNode {
    //                     val: 5,
    //                     left: None,
    //                     right: None,
    //                 }))),
    //             }))),
    //         })))
    //     ),
    //     2
    // );
}