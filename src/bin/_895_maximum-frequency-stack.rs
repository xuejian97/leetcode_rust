/*
Design a stack-like data structure to push elements to the stack and pop the most frequent element from the stack.

Implement the FreqStack class:

FreqStack() constructs an empty frequency stack.
void push(int val) pushes an integer val onto the top of the stack.
int pop() removes and returns the most frequent element in the stack.
If there is a tie for the most frequent element, the element closest to the stack's top is removed and returned.


Example 1:

Input
["FreqStack", "push", "push", "push", "push", "push", "push", "pop", "pop", "pop", "pop"]
[[], [5], [7], [5], [7], [4], [5], [], [], [], []]
Output
[null, null, null, null, null, null, null, 5, 7, 5, 4]

Explanation
FreqStack freqStack = new FreqStack();
freqStack.push(5); // The stack is [5]
freqStack.push(7); // The stack is [5,7]
freqStack.push(5); // The stack is [5,7,5]
freqStack.push(7); // The stack is [5,7,5,7]
freqStack.push(4); // The stack is [5,7,5,7,4]
freqStack.push(5); // The stack is [5,7,5,7,4,5]
freqStack.pop();   // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
freqStack.pop();   // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
freqStack.pop();   // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
freqStack.pop();   // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].


Constraints:

0 <= val <= 109
At most 2 * 104 calls will be made to push and pop.
It is guaranteed that there will be at least one element in the stack before calling pop.
 */
use std::collections::HashMap;

struct FreqStack {
    freq_stack_map: HashMap<usize, Vec<i32>>,
    val_freq_map: HashMap<i32, usize>,
    max_freq: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        FreqStack {
            freq_stack_map: Default::default(),
            val_freq_map: Default::default(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        if !self.val_freq_map.contains_key(&val) {
            self.val_freq_map.insert(val, 0);
        }
        let freq = self.val_freq_map.get_mut(&val).unwrap();
        *freq += 1;

        if !self.freq_stack_map.contains_key(freq) {
            self.freq_stack_map.insert(*freq, vec![]);
        }
        let freq_stack = self.freq_stack_map.get_mut(freq).unwrap();
        freq_stack.push(val);

        if *freq > self.max_freq {
            self.max_freq = *freq;
        }
    }

    fn pop(&mut self) -> i32 {
        let stack = self.freq_stack_map.get_mut(&self.max_freq).unwrap();
        let val = stack.pop().unwrap();
        if stack.len() == 0 {
            self.max_freq -= 1;
        }
        *self.val_freq_map.get_mut(&val).unwrap() -= 1;
        val
    }
}

fn main() {}