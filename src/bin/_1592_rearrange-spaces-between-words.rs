/*
You are given a string text of words that are placed among some number of spaces. Each word consists of one or more lowercase English letters and are separated by at least one space. It's guaranteed that text contains at least one word.

Rearrange the spaces so that there is an equal number of spaces between every pair of adjacent words and that number is maximized. If you cannot redistribute all the spaces equally, place the extra spaces at the end, meaning the returned string should be the same length as text.

Return the string after rearranging the spaces.

 

Example 1:

Input: text = "  this   is  a sentence "
Output: "this   is   a   sentence"
Explanation: There are a total of 9 spaces and 4 words. We can evenly divide the 9 spaces between the words: 9 / (4-1) = 3 spaces.
Example 2:

Input: text = " practice   makes   perfect"
Output: "practice   makes   perfect "
Explanation: There are a total of 7 spaces and 3 words. 7 / (3-1) = 3 spaces plus 1 extra space. We place this extra space at the end of the string.
 

Constraints:

1 <= text.length <= 100
text consists of lowercase English letters and ' '.
text contains at least one word.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/rearrange-spaces-between-words
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut words: Vec<String> = vec!["".to_owned()];
        let mut space_cnt: usize = 0;

        text.as_bytes().iter()
            .map(|u| *u as char)
            .for_each(|c| {
                match c {
                    ' ' => {
                        if words.last_mut().unwrap().len() > 0 {
                            words.push("".to_owned());
                        }
                        space_cnt += 1;
                    }
                    _ => {
                        words.last_mut().unwrap().push(c);
                    }
                }
            });

        if words.last().unwrap().len() == 0 {
            words.pop();
        }
        let mut res;
        let tail_spaces_length;
        if words.len() > 1 {
            let separator = vec![" "; space_cnt / (words.len() - 1)].join("");
            res = words.join(&separator);
            tail_spaces_length = space_cnt % (words.len() - 1);
        } else {
            res = words[0].clone();
            tail_spaces_length = space_cnt;
        }

        if tail_spaces_length > 0 {
            for _ in 0..tail_spaces_length {
                res.push(' ');
            }
        }

        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::reorder_spaces("  this   is  a sentence ".to_owned()),
        "this   is   a   sentence".to_owned()
    );
    assert_eq!(
        Solution::reorder_spaces("a".to_owned()),
        "a".to_owned()
    );
}