/*
Given the root of a binary tree, return the maximum width of the given tree.

The maximum width of a tree is the maximum width among all levels.

The width of one level is defined as the length between the end-nodes (the leftmost and rightmost non-null nodes), where the null nodes between the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.

It is guaranteed that the answer will in the range of a 32-bit signed integer.

 

Example 1:


Input: root = [1,3,2,5,3,null,9]
Output: 4
Explanation: The maximum width exists in the third level with length 4 (5,3,null,9).
Example 2:


Input: root = [1,3,2,5,null,null,9,6,null,7]
Output: 7
Explanation: The maximum width exists in the fourth level with length 7 (6,null,null,null,null,null,7).
Example 3:


Input: root = [1,3,2,5]
Output: 2
Explanation: The maximum width exists in the second level with length 2 (3,2).
 

Constraints:

The number of nodes in the tree is in the range [1, 3000].
-100 <= Node.val <= 100

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-width-of-binary-tree
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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let mut max_width = 0;
        let mut queue: VecDeque<(usize, Option<Rc<RefCell<TreeNode>>>)> = VecDeque::new();
        queue.push_back((0, root));

        while queue.len() > 0 {
            let layer_width = queue.back().unwrap().0 - queue.front().unwrap().0 + 1;
            if layer_width > max_width { max_width = layer_width }
            for _ in 0..queue.len() {
                let (index, n) = queue.pop_front().unwrap();
                let n = n.as_ref().unwrap().borrow();
                if n.left.is_some() {
                    queue.push_back((index * 2, n.left.clone()));
                }
                if n.right.is_some() {
                    queue.push_back((index * 2 + 1, n.right.clone()));
                }
            }
        }

        max_width as i32
    }
}

struct Solution;

fn main() {
    let root = Some(Rc::new(RefCell::new(
        TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(
                        TreeNode {
                            val: 5,
                            left: Some(Rc::new(RefCell::new(
                                TreeNode {
                                    val: 6,
                                    left: None,
                                    right: None,
                                }
                            ))),
                            right: None,
                        }
                    ))),
                    right: None,
                }
            ))),
            right: Some(Rc::new(RefCell::new(
                TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(
                        TreeNode {
                            val: 9,
                            left: Some(Rc::new(RefCell::new(
                                TreeNode {
                                    val: 7,
                                    left: None,
                                    right: None,
                                }
                            ))),
                            right: None,
                        }
                    ))),
                }
            ))),
        }
    )));

    assert_eq!(Solution::width_of_binary_tree(root), 7);
}