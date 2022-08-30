/*
Given two integer arrays pushed and popped each with distinct values, return true if this could have been the result of a sequence of push and pop operations on an initially empty stack, or false otherwise.

 

Example 1:

Input: pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
Output: true
Explanation: We might do the following sequence:
push(1), push(2), push(3), push(4),
pop() -> 4,
push(5),
pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
Example 2:

Input: pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
Output: false
Explanation: 1 cannot be popped before 2.
 

Constraints:

1 <= pushed.length <= 1000
0 <= pushed[i] <= 1000
All the elements of pushed are unique.
popped.length == pushed.length
popped is a permutation of pushed.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/validate-stack-sequences
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        let mut popped_index = 0;
        for &x in &pushed {
            stack.push_back(x);
            while stack.back().is_some() && stack.back().unwrap() == &popped[popped_index] {
                stack.pop_back();
                popped_index += 1;
            }
        }

        popped_index == pushed.len()
    }

    pub fn validate_stack_sequences_2(mut pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut popped_index = 0;
        for i in 0..pushed.len() {
            for j in (0..=i).rev() {
                if pushed[j as usize] == -1 { continue; }
                if pushed[j as usize] != popped[popped_index] { break; }
                pushed[j as usize] = -1;
                popped_index += 1;
            }
        }
        popped_index == pushed.len()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1],
        ),
        true
    );
    assert_eq!(
        Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 3, 5, 1, 2],
        ),
        false
    );


    assert_eq!(
        Solution::validate_stack_sequences_2(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1],
        ),
        true
    );
    assert_eq!(
        Solution::validate_stack_sequences_2(
            vec![1, 2, 3, 4, 5],
            vec![4, 3, 5, 1, 2],
        ),
        false
    );
}