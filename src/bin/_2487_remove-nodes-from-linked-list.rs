/*
You are given the head of a linked list.

Remove every node which has a node with a strictly greater value anywhere to the right side of it.

Return the head of the modified linked list.



Example 1:


Input: head = [5,2,13,3,8]
Output: [13,8]
Explanation: The nodes that should be removed are 5, 2 and 3.
- Node 13 is to the right of node 5.
- Node 13 is to the right of node 2.
- Node 8 is to the right of node 3.
Example 2:

Input: head = [1,1,1,1]
Output: [1,1,1,1]
Explanation: Every node has value 1, so no nodes are removed.


Constraints:

The number of the nodes in the given list is in the range [1, 105].
1 <= Node.val <= 105
 */
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = vec![];
        let mut pointer = &head;
        while let Some(node) = pointer {
            values.push(node.val);
            pointer = &node.next;
        }
        let mut tail_max = 0;
        let mut new_values = vec![];
        for i in (0..values.len()).rev() {
            if values[i] >= tail_max {
                tail_max = values[i];
                new_values.push(values[i])
            }
        }
        let mut next = None;
        for x in new_values {
            let node = Some(Box::new(ListNode { val: x, next }));
            next = node;
        }

        next
    }
}

struct Solution;

fn main() {
}