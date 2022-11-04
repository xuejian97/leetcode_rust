/*
For a string sequence, a string word is k-repeating if word concatenated k times is a substring of sequence. The word's maximum k-repeating value is the highest value k where word is k-repeating in sequence. If word is not a substring of sequence, word's maximum k-repeating value is 0.

Given strings sequence and word, return the maximum k-repeating value of word in sequence.



Example 1:

Input: sequence = "ababc", word = "ab"
Output: 2
Explanation: "abab" is a substring in "ababc".
Example 2:

Input: sequence = "ababc", word = "ba"
Output: 1
Explanation: "ba" is a substring in "ababc". "baba" is not a substring in "ababc".
Example 3:

Input: sequence = "ababc", word = "ac"
Output: 0
Explanation: "ac" is not a substring in "ababc".


Constraints:

1 <= sequence.length <= 100
1 <= word.length <= 100
sequence and word contains only lowercase English letters.
 */

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let option = sequence.find(word.as_str());
        if option.is_none() { return 0; }

        let sequence_bytes = sequence.as_bytes();
        let word_bytes = word.as_bytes();
        let mut sub_str_index_vec = vec![];
        'outer: for i in option.unwrap()..sequence.len() {
            if sequence_bytes[i] != word_bytes[0] { continue; }
            for j in 1..word.len() {
                if i + j > sequence_bytes.len() - 1 { continue 'outer; }
                if sequence_bytes[i + j] != word_bytes[j] {
                    continue 'outer;
                }
            }
            sub_str_index_vec.push(i);
        }

        let mut max_k = 1;
        let mut cur_k = 1;
        while sub_str_index_vec.len() > 0 {
            let mut pre = sub_str_index_vec[0];
            sub_str_index_vec.remove(0);
            while let Ok(i) = sub_str_index_vec.binary_search(&(pre + word.len())) {

                pre = sub_str_index_vec[i];
                sub_str_index_vec.remove(i);
                cur_k += 1;
            }
            max_k = max_k.max(cur_k);
            cur_k = 1;
        }
        max_k.max(cur_k)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_repeating("ababc".to_owned(), "ab".to_owned()), 2);
    assert_eq!(Solution::max_repeating("aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_owned(), "aaaba".to_owned()), 5);
    assert_eq!(Solution::max_repeating("aaaaaa".to_owned(), "aa".to_owned()), 3);
}