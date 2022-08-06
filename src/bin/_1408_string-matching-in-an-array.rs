/*
Given an array of string words. Return all strings in words which is substring of another word in any order. 

String words[i] is substring of words[j], if can be obtained removing some characters to left and/or right side of words[j].

 

Example 1:

Input: words = ["mass","as","hero","superhero"]
Output: ["as","hero"]
Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
["hero","as"] is also a valid answer.
Example 2:

Input: words = ["leetcode","et","code"]
Output: ["et","code"]
Explanation: "et", "code" are substring of "leetcode".
Example 3:

Input: words = ["blue","green","bu"]
Output: []
 

Constraints:

1 <= words.length <= 100
1 <= words[i].length <= 30
words[i] contains only lowercase English letters.
It's guaranteed that words[i] will be unique.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/string-matching-in-an-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {

        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut result: Vec<String> = vec![];

        let mut candidate: Vec<String> = vec![];

        for i in 0..words.len() {
            candidate = candidate.iter()
                .filter(|&x| {
                    if words[i].contains(x) {
                        result.push(x.clone());
                        false
                    } else {
                        true
                    }
                })
                .map(|x|  x.to_owned() )
                .collect();

            candidate.push(words[i].clone());
        }

        result
    }
}

struct Solution;

fn main() {
    let words: Vec<String> = vec!["mass".to_owned(), "as".to_owned(), "hero".to_owned(), "superhero".to_owned()];
    let result = Solution::string_matching(words);
    println!("{:?}", result);
}