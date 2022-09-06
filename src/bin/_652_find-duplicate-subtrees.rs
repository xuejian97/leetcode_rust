/*
Given the root of a binary tree, return all duplicate subtrees.

For each kind of duplicate subtrees, you only need to return the root node of any one of them.

Two trees are duplicate if they have the same structure with the same node values.

 

Example 1:


Input: root = [1,2,3,4,null,2,4,null,null,4]
Output: [[2,4],[4]]
Example 2:


Input: root = [2,1,1]
Output: [[1]]
Example 3:


Input: root = [2,2,2,3,null,3,null]
Output: [[2,3],[3]]
 

Constraints:

The number of the nodes in the tree will be in the range [1, 10^4]
-200 <= Node.val <= 200


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-duplicate-subtrees
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
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        use std::collections::HashMap;
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut repeat = vec![];

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, func: &mut impl FnMut(&str, Option<Rc<RefCell<TreeNode>>>)) -> String {
            match root {
                Some(node) => {
                    let mut serial = node.borrow().val.to_string();
                    serial.push_str("(");
                    serial.push_str(&dfs(node.borrow().left.clone(), func));
                    serial.push_str(")(");
                    serial.push_str(&dfs(node.borrow().right.clone(), func));
                    serial.push_str(")");
                    func(&serial, Some(node));
                    serial
                }
                None => "".to_owned()
            }
        }

        dfs(root, &mut |serial: &str, node: Option<Rc<RefCell<TreeNode>>>| {
            match map.get_mut(serial) {
                Some(v) => {
                    *v += 1;
                    if *v == 2 {
                        repeat.push(node.clone());
                    }
                }
                None => {
                    map.insert(serial.to_owned(), 1);
                }
            }
        });

        repeat
    }
}

struct Solution;

fn main() {
    // [0,0,0,0,null,null,0,null,null,null,0]
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));
    // Some(Rc::new(RefCell::new(TreeNode{})))

    let vec = Solution::find_duplicate_subtrees(root);
    println!("{:?}", vec);
}